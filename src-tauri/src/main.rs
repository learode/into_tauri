#![cfg_attr(all(not(debug_assertions), target_os="windows"), windows_subsystem = "windows")]
use tauri::{Menu, MenuItem, CustomMenuItem, Builder, MenuEntry, Submenu, Manager};

/**
 * Trying to understand the use of Menu, Menu::with_items, MenuEntry.
 * Its implemented as using the native menu items but I donot understand why its not working.
 * 
 * Only about is unsupported
 */

 #[tauri::command]
 fn greet(name: &str) -> String {
    format!("Yo nigga {name}, whats up?")
 }


fn main() {
    Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .menu(Menu::with_items([
            MenuEntry::Submenu(Submenu::new(
                "File", 
                Menu::with_items([
                    CustomMenuItem::new("new", "New").into(),
                    CustomMenuItem::new("open", "Open").into(),
                    CustomMenuItem::new("more", "More").into(),
                ])
            ))
        ]))
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "new" => { println!("Still to learn how :sobbing:") },
                "more" => { 
                    tauri::api::shell::open(&event.window().shell_scope(), "https://github.com/learode/liferoute", None).unwrap()
                },
                _ => { println!("Jez Rust is a pain, I just forgot this statement You have to complain niga?");  }
            }
        })
        .run(tauri::generate_context!())
        .expect("Wow! Erro while trying to build a window :ha:");
    ()
}