use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn open_app_window(
    app: &AppHandle,
    label: &str,
    url: &str,
    title: &str,
    width: f64,
    height: f64,
) -> Result<(), String> {
    if let Some(window) = app.get_webview_window(label) {
        window.set_focus().map_err(|error| error.to_string())?;
        return Ok(());
    }

    WebviewWindowBuilder::new(app, label, WebviewUrl::App(url.into()))
        .title(title)
        .inner_size(width, height)
        .build()
        .map_err(|error| error.to_string())?;

    Ok(())
}

#[tauri::command]
fn open_query_window(app: AppHandle) -> Result<(), String> {
    open_app_window(&app, "query", "query.html", "SQL Console", 960.0, 640.0)
}

#[tauri::command]
fn open_settings_window(app: AppHandle) -> Result<(), String> {
    open_app_window(&app, "settings", "settings.html", "Settings", 720.0, 520.0)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            open_query_window,
            open_settings_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
