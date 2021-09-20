#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

#[tauri::command]
fn editor_instance_init() {
  // <editor> svelte component/window has just been created.
  println!("Editor new instance triggerd");
}

fn main() {

  // Desktop tray
  let exit_item = CustomMenuItem::new("exit".to_string(), "Exit");
  let tray_menu = SystemTrayMenu::new().add_item(exit_item);
  
  let tray = SystemTray::new().with_menu(tray_menu);

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![editor_instance_init])
    .system_tray(tray)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
