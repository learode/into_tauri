#![cfg_attr(all(not(debug_assertions), target_os="windows"), windows_subsystem = "windows")]
use tauri::{Menu, MenuItem, Submenu, CustomMenuItem };




#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}


fn main() {
    // Menu items instances
    // define menu for file related action
    let quit = CustomMenuItem::new("quit", "Quit");
    let close = CustomMenuItem::new("close", "Close");

    let fileSubmenu = Submenu::new("File", Menu::new().add_item(quit)
                        .add_item(close));
    
    // Menu for Help, with sub being about, release note and video tutorial
    let release_note_mi = CustomMenuItem::new("release_note", "Release Notes");
    let video_tutorial_mi = CustomMenuItem::new("video_tutorials", "Video Tutorials");
    let about_mi = CustomMenuItem::new("about", "About Us");

    let helpSubmenu = Submenu::new("Help", Menu::new()
                            .add_item(release_note_mi)
                            .add_item(video_tutorial_mi)
                            .add_item(about_mi)
                        );

    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        // add menu context
        // .add_item(CustomMenuItem::new("hide", "Hide"))
        // Add a menu with its content as defined
        .add_submenu(fileSubmenu)
        .add_submenu(helpSubmenu);



    tauri::Builder::default()
        .menu(menu)
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("Error while initialising tauri application");
}