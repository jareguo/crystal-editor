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
  
  // TODO: windows 11 not showing tray icon
  let tray = SystemTray::new().with_menu(tray_menu).with_icon(tauri::Icon::Raw(include_bytes!("../icons/icon.png").to_vec()));

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![editor_instance_init])
    .system_tray(tray)
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::MenuItemClick { id, .. } => {
        //let item_handle = app.tray_handle().get_item(&id);
        match id.as_str() {
          "exit" => {
            let window = app.get_window("main").unwrap();
            match window.close(){
              Ok(_) => {
                println!("Window closed");
              },
              Err(e) => {
                println!("Error closing window: {}", e);
              }
            }
            std::process::exit(0);
          }
          _ => {}
        }
      }
      _ => {}
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
