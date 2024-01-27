// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    AppHandle, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

fn main() {
    let quit_item = CustomMenuItem::new("quit", "Quit").accelerator("cmd+q");
    let settings_item = CustomMenuItem::new("settings", "Settings...").accelerator("cmd+,");

    let tray_menu = SystemTrayMenu::new()
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(settings_item)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit_item);

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(on_tray_event)
        // .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn on_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    if let SystemTrayEvent::MenuItemClick { id, .. } = event {
        let _item_handle = app.tray_handle().get_item(&id);
        match id.as_str() {
            "settings" => {
                eprintln!("TODO: Settings Window");
                return;
            }
            "quit" => app.exit(0),
            _ => return,
        }
    }
}
