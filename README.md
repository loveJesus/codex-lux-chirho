# Codex Lux Chirho

> For God so loved the world that he gave his only begotten Son,
> that whoever believes in him should not perish but have eternal life.
> — John 3:16

**Codex Lux Chirho** (Latin: "Codex of Light") is a cross-platform Bible study application built with Rust and Slint UI framework, powered by rsword_chirho - a pure Rust SWORD library port.

## Features Chirho

### Current (v0.1.8)

- **Multi-Module Support Chirho**: Load and display SWORD Bible modules via rsword_chirho (RawText, zText, RawCom, zCom, RawLD, zLD formats)
- **Navigation Chirho**: Browse all 66 books with Old/New Testament grouping, chapter grid selector, prev/next book buttons
- **Goto Verse Chirho**: Jump to any verse with Ctrl+G (supports abbreviations like "Jn 3:16", "Gen 1:1", "1Cor 13:4")
- **Text Display Chirho**: Red letter text support, section headings, poetry indentation (0-3 levels), paragraph markers
- **Text Rendering Chirho**: Proper rendering of Hebrew (RTL with vowel points) and Greek (with accents)
- **Highlights Chirho**: Click verses to highlight them (persisted in SQLite, 5 colors, filter by color)
- **Notes Chirho**: Personal notes on verses with save/delete UI, note indicator on verses
- **Bookmarks Chirho**: Bookmark verses with sidebar panel, quick navigation to bookmarked verses, edit labels
- **Reading History Chirho**: Track and navigate to recently read chapters
- **Search Chirho**: Case-insensitive text search with scope selection (All, OT, NT, Current Book)
- **Data Persistence Chirho**: SQLite database for highlights, bookmarks, notes, and settings
- **State Restoration Chirho**: Remembers your last location between sessions
- **Theme Toggle Chirho**: Dark, Light, Sepia, and High Contrast themes with Ctrl+T shortcut
- **Font Scaling Chirho**: Adjustable scripture text size, line height, and font family
- **Settings Panel Chirho**: Theme, font, paragraph mode, data export, and keyboard shortcuts reference
- **Module Manager Chirho**: Browse available SWORD modules, view install status (Ctrl+M)
- **Quick Navigation Chirho**: Command palette style navigation with Ctrl+K (fuzzy book search, recent history)
- **Context Menu Chirho**: Right-click verses for quick actions (copy, bookmark, highlight, note, compare translations)
- **Copy Format Options Chirho**: Configurable copy format (Text-Ref, Ref:Text, Text Only) with verse number toggle
- **Data Import/Export Chirho**: Export/import highlights, bookmarks, notes to JSON for backup and restore
- **First-Run Onboarding Chirho**: Welcome wizard with feature tour and keyboard shortcuts guide
- **Accessibility Chirho**: Reduce motion option, focus indicators toggle, high contrast theme
- **Parallel View Chirho**: Side-by-side translation comparison with Ctrl+P (select second module to compare)
- **Info Panel Chirho**: Contextual verse details sidebar with Ctrl+I (verse reference, text, highlight/bookmark/note status)
- **Statistics Chirho**: Reading progress tracking with Ctrl+Y (chapters read, reading streaks, study activity counts)
- **Study Journal Chirho**: Personal Bible study journal with Ctrl+J (create entries, link to verses, add tags)
- **Prayer List Chirho**: Prayer request tracking with Ctrl+R (add requests, link to verses, mark answered, categories)
- **Verse of the Day Chirho**: Daily devotional verse with 🌅 toolbar button (31+ curated verses)
- **Strong's Numbers Chirho**: Hebrew/Greek Strong's concordance lookup with H button and popup definitions
- **Morphology Display Chirho**: Greek and Hebrew grammatical analysis with M button (Robinson/OSHM parsing)
- **Cross-References Chirho**: Scripture cross-reference display with ⤴ button (clickable refs, popup preview)
- **Footnotes Chirho**: Translator and textual footnotes with ※ button (alternative readings, variant notes)
- **Reading Plans Chirho**: Pre-built Bible reading plans with 📅 toolbar button (1-year, 90-day NT, Gospels, Psalms/Proverbs, Chronological)
- **Commentary Chirho**: Commentary integration with 📖 toolbar button (Ctrl+Shift+C) - auto-sync with current verse, module selection
- **Lexicon Chirho**: Dictionary/lexicon lookup with 📚 toolbar button (Ctrl+Shift+L) - search, A-Z browse, entry history
- **Interlinear Chirho**: Word-by-word Hebrew/Greek display with ≡ toolbar button (Ctrl+Shift+I) - original text, transliteration, morphology, Strong's, gloss
- **Keyboard Shortcuts Chirho**: Navigate quickly with keyboard (28+ shortcuts)

## Keyboard Shortcuts Chirho

| Shortcut | Action |
|----------|--------|
| `←` / `→` | Previous / Next chapter |
| `‹‹` / `››` | Previous / Next book (toolbar) |
| `Ctrl+F` | Toggle search panel |
| `Ctrl+G` | Goto verse dialog |
| `Ctrl+K` | Quick navigation (command palette) |
| `Ctrl+B` | Toggle books sidebar |
| `Ctrl+D` | Toggle bookmarks panel |
| `Ctrl+H` | Toggle highlights panel |
| `Ctrl+I` | Toggle info panel (verse details) |
| `Ctrl+N` | Toggle notes panel |
| `Ctrl+M` | Toggle module manager panel |
| `Ctrl+P` | Toggle parallel view (side-by-side translations) |
| `Ctrl+T` | Cycle themes (Dark → Light → Sepia → High Contrast) |
| `Ctrl+Y` | Toggle statistics panel (reading progress) |
| `Ctrl+J` | Toggle study journal panel |
| `Ctrl+R` | Toggle prayer list panel |
| `Ctrl+Shift+V` | Show Verse of the Day dialog |
| `Ctrl+Shift+S` | Toggle Strong's Numbers display |
| `Ctrl+Shift+M` | Toggle Morphology display |
| `Ctrl+Shift+X` | Toggle Cross-References display |
| `Ctrl+Shift+F` | Toggle Footnotes display |
| `Ctrl+Shift+P` | Toggle Reading Plans panel |
| `Ctrl+Shift+C` | Toggle Commentary panel |
| `Ctrl+Shift+L` | Toggle Lexicon panel |
| `Ctrl+Shift+I` | Toggle Interlinear panel |
| `Ctrl+,` | Toggle settings panel |
| `Escape` | Close dialogs/panels |

### Planned Features Chirho

See [PRD_CHIRHO.json](PRD_CHIRHO.json) for the complete product roadmap with 55+ features including:

- Full-text search with Tantivy indexing
- Strong's numbers and morphology display
- Cross-references and footnotes
- Parallel view (multiple translations)
- Interlinear display with word-by-word gloss
- Reading plans and devotionals
- Commentary integration
- Module management (install/uninstall from CrossWire)
- Mac App Store, iOS, and Android distribution

## Tech Stack Chirho

- **Language**: Rust
- **UI Framework**: [Slint](https://slint.dev/) - Native cross-platform UI
- **Bible Engine**: rsword_chirho - Pure Rust SWORD library port with full binary format support
- **Database**: SQLite via rusqlite
- **Search**: Tantivy full-text search engine

## rsword_chirho SWORD Library Chirho

Codex Lux Chirho is powered by rsword_chirho, a pure Rust port of the SWORD Bible software library featuring:

- **Full Binary Format Support Chirho**: RawText, RawText4, zText (compressed) modules
- **Commentary Support Chirho**: RawCom, zCom formats
- **Lexicon Support Chirho**: RawLD, zLD dictionary formats
- **Compression Chirho**: zlib, bzip2, and xz decompression
- **Versification Chirho**: KJV, Catholic, LXX, and other versification systems
- **CLI Tools Chirho**: diatheke_chirho, installmgr_chirho, and module creation tools
- **Module Management Chirho**: Install from CrossWire and other SWORD repositories

## Building Chirho

### Prerequisites Chirho

- Rust 1.70 or later
- Cargo

### Build & Run Chirho

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

## Project Structure Chirho

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

## Data Storage Chirho

User data is stored in a SQLite database at:
- **macOS**: `~/Library/Application Support/com.rsword.codex-lux/codex_lux_chirho.db`
- **Linux**: `~/.local/share/codex-lux/codex_lux_chirho.db`
- **Windows**: `%APPDATA%\rsword\codex-lux\codex_lux_chirho.db`

The database stores:
- Highlights Chirho (verse location, color, timestamp)
- Bookmarks Chirho (verse location, label, folder)
- Notes Chirho (verse location, content, timestamps)
- Settings Chirho (key-value pairs)
- Reading History Chirho (navigation log)

## Naming Convention Chirho

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

## License Chirho

GPL-2.0-or-later

## Contributing Chirho

Contributions welcome! Please ensure:
- All new code follows the Chirho naming convention
- John 3:16 comment header on all source files
- Tests for new functionality
- No secrets in git (use `.env` files)

## Credits Chirho

- SWORD Project for the module format specification
- Slint UI framework
- rsword_chirho Rust SWORD library

---

*Soli Deo Gloria*
