# Tauri + Vanilla

## Invoke Commands

Commands Tauri are essentially Rust functions that enhance your frontend with native capabilities you can call from your frontend JavaScript

### Defining Commands 

Just another Rust function, with the line `#[taur::command]` above it. The function can take any number of arguments, and return any type that implements `serde::Serialize`.

`src-tauri/main.js`

```rust
    #[tauri::command]
    fn greet(name: &str) -> String {
        format!("Hello, {}!", name)
    }
```

If the tauri is import into the project's scope `#[tauri::command]` becomes `#[command]`.

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
