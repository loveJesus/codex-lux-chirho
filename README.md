# Codex Lux Chirho

> For God so loved the world that he gave his only begotten Son,
> that whoever believes in him should not perish but have eternal life.
> — John 3:16

**Codex Lux** (Latin: "Codex of Light") is a cross-platform Bible study application built with Rust and Slint UI framework.

## Features

### Current (v0.1.4)

- **Multi-Module Support**: Load and display SWORD Bible modules via rsword_chirho
- **Navigation**: Browse all 66 books with Old/New Testament grouping, chapter grid selector
- **Goto Verse**: Jump to any verse with Ctrl+G (supports abbreviations like "Jn 3:16", "Gen 1:1", "1Cor 13:4")
- **Text Display**: Proper rendering of Hebrew (RTL with vowel points) and Greek (with accents)
- **Highlights**: Click verses to highlight them (persisted in SQLite, multiple colors supported)
- **Notes**: Personal notes on verses with save/delete UI, note indicator on verses
- **Bookmarks**: Bookmark verses with sidebar panel, quick navigation to bookmarked verses
- **Reading History**: Track and navigate to recently read chapters
- **Search**: Case-insensitive text search with results preview
- **Data Persistence**: SQLite database for highlights, bookmarks, notes, and settings
- **State Restoration**: Remembers your last location between sessions
- **Theme Toggle**: Dark and Light themes with Ctrl+T shortcut
- **Font Scaling**: Adjustable scripture text size and line height
- **Settings Panel**: Theme, font, paragraph mode, and keyboard shortcuts reference
- **Keyboard Shortcuts**: Navigate quickly with keyboard

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `←` / `→` | Previous / Next chapter |
| `Ctrl+F` | Toggle search panel |
| `Ctrl+G` | Goto verse dialog |
| `Ctrl+B` | Toggle books sidebar |
| `Ctrl+D` | Toggle bookmarks panel |
| `Ctrl+H` | Toggle highlights panel |
| `Ctrl+N` | Toggle notes panel |
| `Ctrl+T` | Toggle dark/light theme |
| `Escape` | Close dialogs/panels |

### Planned Features

See [PRD_CHIRHO.json](PRD_CHIRHO.json) for the complete product roadmap with 55+ features including:

- Full-text search with Tantivy
- Strong's numbers and morphology
- Cross-references and footnotes
- Parallel view (multiple translations)
- Interlinear display with word-by-word gloss
- Bookmarks, notes, and reading plans
- Commentary integration
- Module management (install/uninstall)
- Mac App Store, iOS, and Android distribution

## Tech Stack

- **Language**: Rust
- **UI Framework**: [Slint](https://slint.dev/) - Native cross-platform UI
- **Bible Engine**: rsword_chirho - Pure Rust SWORD library port
- **Database**: SQLite via rusqlite
- **Search**: Tantivy full-text search engine

## Building

### Prerequisites

- Rust 1.70 or later
- Cargo

### Build & Run

```bash
# Development build
cargo build

# Run the application
cargo run

# Release build
cargo build --release

# Run tests
cargo test

# Run clippy
cargo clippy
```

## Project Structure

```
codex_lux_chirho/
├── Cargo.toml              # Rust dependencies
├── build.rs                # Slint UI compilation
├── PRD_CHIRHO.json         # Product requirements document
├── README.md               # This file
├── src/
│   └── main.rs             # Application entry point and backend
│       ├── database_chirho # SQLite database module
│       └── bible_engine_chirho # rsword_chirho integration
└── ui_chirho/
    └── main_chirho.slint   # Slint UI definition
```

## Data Storage

User data is stored in a SQLite database at:
- **macOS**: `~/Library/Application Support/com.rsword.codex-lux/codex_lux_chirho.db`
- **Linux**: `~/.local/share/codex-lux/codex_lux_chirho.db`
- **Windows**: `%APPDATA%\rsword\codex-lux\codex_lux_chirho.db`

The database stores:
- Highlights (verse location, color, timestamp)
- Bookmarks (verse location, label, folder)
- Notes (verse location, content, timestamps)
- Settings (key-value pairs)
- Reading history (navigation log)

## Naming Convention

All identifiers follow the Chirho suffix convention:

### Rust (snake_case + _chirho)
- Variables: `variable_name_chirho`
- Functions: `function_name_chirho`
- Structs: `StructNameChirho`
- Constants: `CONSTANT_NAME_CHIRHO`

### Slint UI (kebab-case + -chirho)
Slint language requires kebab-case for identifiers:
- Properties: `property-name-chirho`
- Callbacks: `callback-name-chirho`
- Components: `ComponentNameChirho`

## License

GPL-2.0-or-later

## Contributing

Contributions welcome! Please ensure:
- All new code follows the Chirho naming convention
- John 3:16 comment header on all source files
- Tests for new functionality
- No secrets in git (use `.env` files)

## Credits

- SWORD Project for the module format specification
- Slint UI framework
- rsword_chirho Rust SWORD library

---

*Soli Deo Gloria*
