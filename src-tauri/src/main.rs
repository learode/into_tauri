#![cfg_attr(all(not(debug_assertions), target_os="windows"), windows_subsystem = "windows")]
use tauri::{Menu, CustomMenuItem};
use tauri::{WindowBuilder};




#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}



fn main() {
  let menu = Menu::new().add_item(CustomMenuItem::new("finish", "Finish")); // configure the menu
  
  tauri::Builder::default()
    .setup(|app| {
      let window = WindowBuilder::new(
        app,
        "main-window".to_string(),
        tauri::WindowUrl::App("index.html".into()),
      )
      .menu(menu)
      .build()?;
      let window_ = window.clone();
      window.on_menu_event(move |event| {
        match event.menu_item_id() {
          "quit" => {
            std::process::exit(0);
          }
          "close" => {
            window_.close().unwrap();
          }
          _ => {}
        }
      });
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}