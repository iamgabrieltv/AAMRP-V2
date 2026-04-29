use std::sync::Mutex;

use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};

struct ClientState {
  client: Mutex<DiscordIpcClient>
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
async fn set_activity(state: tauri::State<'_, ClientState>, title: String, artist: String, album: String, large_image: String, small_image: String) -> Result<(), String> {
    let mut client = state.client.lock().unwrap();
    let _ = client.set_activity(activity::Activity::new()
        .activity_type(activity::ActivityType::Listening)
        .status_display_type(activity::StatusDisplayType::State)
        .state(&artist)
        .details(title)
        .assets(activity::Assets::new()
            .large_image(large_image)
            .large_text(album)
            .small_image(small_image)
            .small_text(&artist)
    )
);

  Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![connect, disconnect, set_activity])
        .manage(ClientState {
            client: Mutex::new(DiscordIpcClient::new("1423726101519274056"))
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
