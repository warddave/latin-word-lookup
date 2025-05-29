# Latin Word Lookup

A modern, desktop application for looking up Latin words with detailed grammatical information and macron support. Built with Tauri and powered by Claude AI.

## Features

- 🔍 **Instant Latin word lookup** with comprehensive dictionary entries
- 📝 **Proper macron support** (ā, ē, ī, ō, ū, ȳ) for accurate Latin representation
- 📚 **Detailed grammatical information** including:
  - Part of speech with full parsing
  - Principal parts for verbs
  - Declension information for nouns
  - Tense, mood, and voice for verb forms
- ⚡ **Offline caching** for previously looked up words (30-day retention)
- 🎨 **Material Design 3** interface
- 🤖 **Powered by Claude AI** for accurate Latin analysis
- 🛠️ **Custom prompt support** to tailor AI responses
- ⌨️ **Keyboard shortcuts** (⌘, for Settings on macOS)

## Requirements

- macOS, Windows, or Linux
- An Anthropic API key (get one at [console.anthropic.com](https://console.anthropic.com))

## Installation

### Download Pre-built Release

Download the latest release for your platform:
- [Latin Word Lookup v0.1.1 (macOS Apple Silicon)](https://github.com/warddave/latin-word-lookup/releases/download/v0.1.1/Latin.Word.Lookup_0.1.1_aarch64.dmg)
  - SHA256: `12f665baf4f1e4b74d57a8da99835bd4916d51a6c5acb62aa7ebd1c305c223ed`

After downloading:
1. Open the DMG file
2. Drag the Latin Word Lookup app to your Applications folder
3. On first launch, you may need to right-click and select "Open" due to macOS security settings

### Build from Source

#### Prerequisites

1. **Rust** (1.75 or later)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Node.js** (optional, only if modifying frontend)
   - The frontend is pre-built in the `dist/` directory

3. **Tauri CLI**
   ```bash
   cargo install tauri-cli
   ```

#### Building the Application

1. Clone the repository:
   ```bash
   git clone https://github.com/warddave/latin-word-lookup.git
   cd latin-word-lookup
   ```

2. Build the application:
   ```bash
   cd src-tauri
   cargo tauri build
   ```

   This will create platform-specific bundles in `src-tauri/target/release/bundle/`:
   - **macOS**: `Latin Word Lookup.app` and `.dmg` installer
   - **Windows**: `.exe` and `.msi` installer
   - **Linux**: `.deb` and `.AppImage`

#### Development Mode

To run the application in development mode:

```bash
cd src-tauri
cargo tauri dev
```

## Usage

1. **First-time setup**: 
   - Launch the application
   - Go to Settings (⌘, on macOS or via menu)
   - Enter your Anthropic API key
   - The key is stored securely on your local machine

2. **Looking up words**: 
   - Type any Latin word and press Enter or click the search button
   - The app automatically adds macrons where needed (e.g., "amicus" → "amīcus")
   - View detailed grammatical information and definitions

3. **Custom prompts**: 
   - In Settings, add additional instructions to customize how words are analyzed
   - For example: "Include etymology information" or "Provide more examples"

4. **Cache management**: 
   - Clear cached lookups from Settings when needed
   - Cache improves performance and works offline

## Development

### Running Tests

Run Rust tests:
```bash
cd src-tauri
cargo test
```

Run integration tests:
```bash
cd src-tauri
cargo test --test integration_tests
```

View JavaScript tests:
- Open `dist/test.html` in a browser

### Project Structure

- `src-tauri/` - Rust backend code
  - `src/main.rs` - Main application entry point
  - `src/claude_api.rs` - Claude API integration
  - `tests/` - Integration tests
- `dist/` - Frontend HTML/CSS/JS
  - `index.html` - Main application UI
  - `test.html` - JavaScript test suite

## Usage

1. Launch the application
2. Enter a Latin word in the text box
3. Press Enter or click "Lookup"
4. View the results with macrons, part of speech, and definition

## Testing

The application includes comprehensive tests:

- **Rust unit tests**: Test input validation and API response handling
- **Rust integration tests**: Test the Claude API client with mocked responses
- **JavaScript tests**: Test UI functionality and error handling

All critical paths are covered by tests to ensure reliability.