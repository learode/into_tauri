# Tauri + Vanilla

## Invoke Commands

Commands in Tauri are essentially Rust functions that enhance your frontend with native capabilities you can call from your frontend JavaScript

### Defining Commands

Just another Rust function, with the line `#[taur::command]` above it. The function can take any number of arguments, and return any type that implements `serde::Serialize`.

`src-tauri/main.js`

```rust
    #[tauri::command]
    fn greet(name: &str) -> String {
        format!("Hello, {}!", name)
    }
```

If the `tauri` (module?) is import into the project's scope `#[tauri::command]` becomes `#[command]`.

This implimented __Command__ is made known to Tauri
Why? So that Tauri can route calls to this __Command__ from __Js__.

```rust
    fn main() -> tauri::Result<()> {
        tauri::Builder::default()
            .invoke_handler(tauri::generate_handler![greet])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
        Ok(())
    }
```

## Two ways of calling Command from the frontend

This will inject a pre-bundled version of the API functions into your frontend.

1. ### Using @tauri-apps/api

    This is best used with a bundler else

2. ### Using withGlobalTauri

    The `withGlobalTauri` can be enable in the `tauri.conf.json` file.

## Menus

menu is a struct that represents a menu bar.
Menus are created using a list struct.

1. Using `Submenu`

    ```rust
        let about_mi = CustomMenuItem::new("about", "About Us"); // *1*
        
        let help_submenu = Submenu::new("Help", Menu::new()
                            .add_item(CustomMenuItem::new("about", "About Us");) // *2*
                            .add_item(about_mi) // *3*
                        );

        let menu = Menu::new().add_submenu(helpSubmenu);
    ```

    \*1*+\*3* does the same thing like \*2*

## Hooking events to menu items

- ### Using `Match`

    ```rs
    fn main () {
        tauri::Builder::default()
            .menu(help_submenu)
            .on_menu_event(|event| {
                match event.menu_item_id() {
                    "quit" => { std::process::exit(0); },
                    "close" => { event.window().close().unwrap(); },
                    _ => {}
                }
            })
    }
    ```

The Implimentation of these Menu guys e.g.

```rs
    impl CustomMenuItem {
      /// Create new custom menu item.
      pub fn new<I: Into<String>, T: Into<String>>(id: I, title: T) -> Self {
        // ...
      }
  }
```

require a String parameter but doesnot complain of a &str slice? :donot know:
I also dont understand the `Into()` guys

The `Into<String>` Takes any string type and convert it into a type `String`. That's why using `&str` does not report and error or complain :ha:

Also the menu items have me click twice before execution, :donot know:
