// Copyright (c) 2025 David Ward
// Licensed under the MIT License

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod claude_api;
mod config;

use claude_api::{ClaudeClient, LatinWordResult};
use config::ConfigManager;
use tauri::{Emitter, Manager, State};
use std::sync::Mutex;

struct AppState {
    config_manager: Mutex<Option<ConfigManager>>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn close_window(window: tauri::Window) -> Result<(), String> {
    window.close().map_err(|e| e.to_string())
}

#[tauri::command]
async fn lookup_latin_word(
    word: String,
    custom_prompt: Option<String>,
    state: State<'_, AppState>,
) -> Result<LatinWordResult, String> {
    // Validate input
    if word.trim().is_empty() {
        return Err("Please enter a word".to_string());
    }
    
    // Allow Latin alphabet characters including macrons (ā, ē, ī, ō, ū, ȳ)
    if !word.chars().all(|c| {
        c.is_alphabetic() && (c.is_ascii() || matches!(c, 'ā' | 'ē' | 'ī' | 'ō' | 'ū' | 'ȳ' | 'Ā' | 'Ē' | 'Ī' | 'Ō' | 'Ū' | 'Ȳ'))
    }) {
        return Err("Please enter only Latin alphabet characters (macrons allowed)".to_string());
    }
    
    // Get API key from config
    let api_key = {
        let state_guard = state.config_manager.lock()
            .map_err(|_| "Failed to acquire lock on config manager")?;
        state_guard.as_ref()
            .ok_or("Config manager not initialized")?
            .get_api_key()
            .ok_or("API key not configured. Please go to Settings to add your Anthropic API key.")?
    };
    
    // Create Claude client and lookup word
    let client = ClaudeClient::new_with_key(api_key);
    client.lookup_latin_word(&word, custom_prompt.as_deref()).await
}

#[tauri::command]
fn save_api_key(
    api_key: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let state_guard = state.config_manager.lock()
        .map_err(|_| "Failed to acquire lock on config manager")?;
    state_guard.as_ref()
        .ok_or("Config manager not initialized")?
        .set_api_key(api_key)
}

#[tauri::command]
fn get_api_key(
    state: State<'_, AppState>,
) -> Result<Option<String>, String> {
    let state_guard = state.config_manager.lock()
        .map_err(|_| "Failed to acquire lock on config manager")?;
    Ok(state_guard.as_ref()
        .ok_or("Config manager not initialized")?
        .get_api_key())
}

#[tauri::command]
fn has_api_key(
    state: State<'_, AppState>,
) -> Result<bool, String> {
    let state_guard = state.config_manager.lock()
        .map_err(|_| "Failed to acquire lock on config manager")?;
    Ok(state_guard.as_ref()
        .ok_or("Config manager not initialized")?
        .has_api_key())
}

#[tauri::command]
fn open_settings(window: tauri::Window) -> Result<(), String> {
    window.emit("open-settings", ()).map_err(|e| e.to_string())
}

#[tauri::command]
fn open_external_url(url: String) -> Result<(), String> {
    open::that(url).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

fn main() {
    let mut builder = tauri::Builder::default();
    
    // Add menu for macOS
    #[cfg(target_os = "macos")]
    {
        use tauri::menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem, SubmenuBuilder};
        
        builder = builder.menu(|handle| {
            MenuBuilder::new(handle)
                .item(
                    &SubmenuBuilder::new(handle, "Latin Word Lookup")
                        .item(&MenuItemBuilder::new("About Latin Word Lookup")
                            .id("about")
                            .build(handle)?)
                        .separator()
                        .item(&MenuItemBuilder::new("Preferences...")
                            .id("preferences")
                            .accelerator("Cmd+,")
                            .build(handle)?)
                        .separator()
                        .item(&PredefinedMenuItem::services(handle, None)?)
                        .separator()
                        .item(&PredefinedMenuItem::hide(handle, None)?)
                        .item(&PredefinedMenuItem::hide_others(handle, None)?)
                        .item(&PredefinedMenuItem::show_all(handle, None)?)
                        .separator()
                        .item(&PredefinedMenuItem::quit(handle, None)?)
                        .build()?)
                .build()
        });
        
        builder = builder.on_menu_event(|handle, event| {
            if let Some(window) = handle.get_webview_window("main") {
                match event.id().as_ref() {
                    "preferences" => {
                        let _ = window.emit("open-settings", ());
                    }
                    "about" => {
                        let _ = window.emit("open-about", ());
                    }
                    _ => {}
                }
            }
        });
    }
    
    builder
        .setup(|app| {
            // Initialize config manager
            let config_manager = ConfigManager::new(&app.handle())?;
            let state = AppState {
                config_manager: Mutex::new(Some(config_manager)),
            };
            app.manage(state);
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            close_window,
            lookup_latin_word,
            save_api_key,
            get_api_key,
            has_api_key,
            open_settings,
            open_external_url,
            get_app_version
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        let result = greet("World");
        assert_eq!(result, "Hello, World! You've been greeted from Rust!");
    }

    #[test]
    fn test_greet_with_name() {
        let result = greet("Alice");
        assert_eq!(result, "Hello, Alice! You've been greeted from Rust!");
    }
}