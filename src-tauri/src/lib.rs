use std::sync::Mutex;

use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use serde_json::Value;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager,
};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, ORIGIN};
use reqwest::Url;

struct ClientState {
    client: Mutex<DiscordIpcClient>,
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
) -> Result<(), String> {
    let mut client = state.client.lock().unwrap();
    let _ = client.set_activity(
        activity::Activity::new()
            .activity_type(activity::ActivityType::Listening)
            .status_display_type(activity::StatusDisplayType::State)
            .state(&artist)
            .details(title)
            .assets(
                activity::Assets::new()
                    .large_image(large_image)
                    .large_text(album)
                    .small_image(small_image)
                    .small_text(&artist),
            ),
    );

    Ok(())
}

#[tauri::command]
async fn apple_request(title: String, artist: String, album: String) -> Result<Value, String> {
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

    let client = reqwest::Client::new();
    let response = client.get(url).headers(headers).send().await.map_err(|e| e.to_string())?;
    let result = response.json::<Value>().await.map_err(|e| e.to_string())?;

    Ok(result)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![connect, disconnect, clear_activity, set_activity, apple_request])
        .manage(ClientState {
            client: Mutex::new(DiscordIpcClient::new("1423726101519274056")),
        })
        .setup(|app| {
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            TrayIconBuilder::new()
                .menu(&menu)
                .icon(app.default_window_icon().unwrap().clone())
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
