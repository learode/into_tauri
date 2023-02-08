#![cfg_attr(all(not(debug_assertions), target_os="windows"), windows_subsystem = "windows")]
use tauri::{Menu, MenuItem, CustomMenuItem, AboutMetadata};

/**
 * Trying to understand the use of MenuItem.
 * Its implemented as using the native menu items but I donot understand why its not working.
 * 
 * Only about is unsupported
 */


fn main() {
    let menu = Menu::new().
                add_native_item(MenuItem::About(
                    // The name passed is appended to the about mkx it ugly :ha:ha:
                    // Without the name, the about has no app name :fustrating:
                    String::from("Liferoute"), 
                    tauri::AboutMetadata::default()
                        .authors(vec![String::from("Mulfranck"), "Tauri".to_string()])
                        .version("0.1.0")
                        .license("MIT")
                        .comments("Knowing Tauri, is a test environment to learn and get a feel of using Tauri for desktop app\n :ha:ha:")
                        .website("https://github.com/learode/liferoutine")
                ))
                .add_native_item(MenuItem::Copy) // Only About is implimemted/supported
                .add_item(CustomMenuItem::new("quit", "Quit"));

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "quit" => { std::process::exit(0); },
                _ => { println!("What? What did you click nigga?"); }
            }
        })
        .run(tauri::generate_context!())
        .expect("Failed to build app");
}