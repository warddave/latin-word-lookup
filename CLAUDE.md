# Playground Project Context

## Project Overview
- Tauri application named "Playground" (previously "playground")
- Located at: /Users/daveward/RustroverProjects/playground/
- Built with Rust backend and web frontend

## Recent Changes
- Changed app name from "playground" to "Playground" in tauri.conf.json (line 9)
- This change affects the macOS application menu display

## Build Information
- Uses Tauri framework for cross-platform desktop app development
- Build command: `cargo tauri build` (requires tauri-cli)
- Alternative Rust-only build: `cargo build --release`
- Configuration file: tauri.conf.json

## Notes
- Tauri CLI installation can be time-consuming due to many dependencies
- The productName in tauri.conf.json controls the displayed application name
- Full Tauri build required to see name changes in macOS menu bar