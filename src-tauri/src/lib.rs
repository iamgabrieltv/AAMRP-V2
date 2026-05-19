use std::sync::Mutex;
use std::time::Duration;

use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, ORIGIN};
use reqwest::{Client, Url};
use serde_json::Value;
use tauri::image::Image;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Emitter, Manager,
};

#[cfg(target_os = "windows")]
use std::sync::Arc;

#[cfg(target_os = "windows")]
use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSessionManager,
    GlobalSystemMediaTransportControlsSessionPlaybackStatus,
};

#[cfg(target_os = "windows")]
#[tauri::command]
async fn get_listening_status_win(state: tauri::State<'_, ClientState>) -> Result<Value, String> {
    let manager = {
        let maybe_manager = state.manager.lock().unwrap().clone();
        if let Some(manager) = maybe_manager {
            manager
        } else {
            let new_manager = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
                .map_err(|e| format!("Failed to request manager: {e}"))?
                .await
                .map_err(|e| format!("Failed to await manager request: {e}"))?;
            let manager = Arc::new(new_manager);
            *state.manager.lock().unwrap() = Some(manager.clone());
            manager
        }
    };

    let sessions = manager
        .GetSessions()
        .map_err(|e| format!("Failed to get sessions: {e}"))?;
    let music_session = sessions
        .into_iter()
        .find(|s| {
            s.SourceAppUserModelId()
                .map_err(|e| format!("Failed to get session app id: {e}"))
                .unwrap()
                .to_string()
                .to_lowercase()
                .contains("applemusic")
        })
        .ok_or_else(|| format!("No Apple Music session found"))
        .map_err(|e| format!("Failed to get session: {e}"))?;

    let status = {
        let playback_info = music_session.GetPlaybackInfo().unwrap();
        match playback_info.PlaybackStatus().unwrap() {
            GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing => true,
            GlobalSystemMediaTransportControlsSessionPlaybackStatus::Paused => false,
            _ => false,
        }
    };
    let media_properties = music_session
        .TryGetMediaPropertiesAsync()
        .unwrap()
        .await
        .unwrap();
    let [artist, album] = media_properties
        .Artist()
        .unwrap()
        .to_string()
        .split(" — ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .try_into()
        .unwrap_or_else(|_| [String::new(), String::new()]);
    let timeline_properties = music_session.GetTimelineProperties().unwrap();

    return Ok(serde_json::json!({
        "title": media_properties.Title().unwrap().to_string(),
        "artist": artist,
        "album": album,
        "is_playing": status,
        "position": timeline_properties.Position().unwrap().Duration / 10000000,
        "duration": timeline_properties.EndTime().unwrap().Duration / 10000000
    }));
}

struct ClientState {
    client: Mutex<DiscordIpcClient>,
    http_client: Mutex<Option<Client>>,
    #[cfg(target_os = "windows")]
    manager: Mutex<Option<Arc<GlobalSystemMediaTransportControlsSessionManager>>>,
}

#[tauri::command]
async fn set_interval(app: AppHandle, interval: u64) -> Result<(), String> {
    tauri::async_runtime::spawn(async move {
        let mut tick = tokio::time::interval(Duration::from_secs(interval));
        loop {
            tick.tick().await;
            app.emit("tick", ());
        }
    });
    Ok(())
}

#[tauri::command]
async fn disconnect(state: tauri::State<'_, ClientState>) -> Result<(), String> {
    let mut client = state.client.lock().unwrap();
    client.clear_activity();
    client.close();

    Ok(())
}

#[tauri::command]
async fn connect(state: tauri::State<'_, ClientState>) -> Result<(), String> {
    let mut client = state.client.lock().unwrap();
    client.connect();

    Ok(())
}

#[tauri::command]
async fn clear_activity(state: tauri::State<'_, ClientState>) -> Result<(), String> {
    let mut client = state.client.lock().unwrap();
    client.clear_activity();

    Ok(())
}

#[tauri::command]
async fn set_activity(
    state: tauri::State<'_, ClientState>,
    title: String,
    artist: String,
    album: String,
    large_image: String,
    small_image: String,
    start_t: i64,
    end_t: i64,
) -> Result<(), String> {
    let mut attempts = 0;

    loop {
        let mut client = state.client.lock().unwrap();
        let result = client.set_activity(
            activity::Activity::new()
                .activity_type(activity::ActivityType::Listening)
                .status_display_type(activity::StatusDisplayType::State)
                .state(&artist)
                .details(title.clone())
                .assets(
                    activity::Assets::new()
                        .large_image(large_image.clone())
                        .large_text(album.clone())
                        .small_image(small_image.clone())
                        .small_text(&artist),
                )
                .timestamps(activity::Timestamps::new().start(start_t).end(end_t)),
        );

        match result {
            Ok(_) => return Ok(()),
            Err(_) => {
                if attempts < 3 {
                    client.connect();
                    drop(client);
                    attempts += 1;
                } else {
                    return Err("Failed to set activity after 3 reconnection attempts".to_string());
                }
            }
        }
    }
}

#[tauri::command]
async fn apple_request(
    state: tauri::State<'_, ClientState>,
    title: String,
    artist: String,
    album: String,
) -> Result<Value, String> {
    let client = {
        let mut http_client = state.http_client.lock().unwrap();
        http_client.get_or_insert_with(Client::new).clone()
    };

    let base_url = "https://amp-api-edge.music.apple.com/v1/catalog/us/search";
    let mut url = Url::parse_with_params(
        base_url,
        &[
            ("l", "en-US"),
            ("limit", "21"),
            ("platform", "web"),
            ("types", "activities,albums,apple-curators,artists,curators,editorial-items,music-movies,music-videos,playlists,record-labels,songs,stations,tv-episodes,uploaded-videos"),
            ("extend", "artistUrl"),
            ("with", "lyricHighlights,lyrics,naturalLanguage,serverBubbles,subtitles"),
        ],
    ).map_err(|e| e.to_string())?;

    let term = format!("{} {} {}", title, album, artist);
    url.query_pairs_mut().append_pair("term", &term);

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_static("Bearer eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6IldlYlBsYXlLaWQifQ.eyJpc3MiOiJBTVBXZWJQbGF5IiwiaWF0IjoxNzc1ODY1MTMwLCJleHAiOjE3ODMxMjI3MzAsInJvb3RfaHR0cHNfb3JpZ2luIjpbImFwcGxlLmNvbSJdfQ.4vZrrfLuSubBlA6_V4k4VH5VVSq6i5xUa_0s1D5oGwaTgxD9M-WotMjMBlqi5M3ktO133nRk2ZncVYGeYP4sUg"));
    headers.insert(ORIGIN, HeaderValue::from_static("https://music.apple.com"));

    let response = client
        .get(url)
        .headers(headers)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let result = response.json::<Value>().await.map_err(|e| e.to_string())?;

    Ok(result)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            connect,
            disconnect,
            clear_activity,
            set_activity,
            apple_request,
            set_interval,
            #[cfg(target_os = "windows")]
            get_listening_status_win
        ])
        .manage(ClientState {
            client: Mutex::new(DiscordIpcClient::new("1423726101519274056")),
            http_client: Mutex::new(None),
            #[cfg(target_os = "windows")]
            manager: Mutex::new(None),
        })
        .setup(|app| {
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;
            let tray_icon: Image = {
                #[cfg(target_os = "macos")]
                {
                    let tray_path = app.path().resolve(
                        "icons/TrayIcon-Template.png",
                        tauri::path::BaseDirectory::Resource,
                    )?;
                    Image::from_path(tray_path)?
                }
                #[cfg(not(target_os = "macos"))]
                {
                    app.default_window_icon().unwrap().clone()
                }
            };

            TrayIconBuilder::new()
                .menu(&menu)
                .icon(tray_icon)
                .icon_as_template(true)
                .tooltip("AAMRP")
                .build(app)?;
            Ok(())
        })
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }
            "show" => {
                let window = app.get_webview_window("main").unwrap();
                if !window.is_visible().unwrap() {
                    window.show().unwrap();
                }
                window.set_focus().unwrap();
            }
            _ => {
                println!("menu item {:?} not handled", event.id);
            }
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                window.hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
