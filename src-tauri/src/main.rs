#![cfg_attr(all(not(debug_assertions), target_os="windows"), windows_subsystem = "windows")]
#[allow(unused_imports)]
use tauri::{Menu, MenuItem, CustomMenuItem, Builder, MenuEntry, Submenu, AboutMetadata};

/**
 * Trying to understand the use of Menu, Menu::with_items, MenuEntry.
 * Its implemented as using the native menu items but I donot understand why its not working.
 * 
 * Only about is unsupported
 */
fn main() {
    let menu = Menu::with_items(vec![
        MenuEntry::Submenu(Submenu::new(
            "File".to_string(),
            Menu::with_items(vec![
                CustomMenuItem::new("new", "New").into(),
                CustomMenuItem::new("open", "Open").into(),
                CustomMenuItem::new("more", "More").into(),
            ])
        )),
        MenuItem::About("About".to_string(), 
            AboutMetadata::new()).into(),
        MenuItem::Separator.into(),
        MenuItem::Quit.into(),
    ]);
    Builder::default()
        .menu(menu)
        .run(tauri::generate_context!())
        .expect("Fail to run app Builder.");
    ()
}