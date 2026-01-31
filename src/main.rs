// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

use std::rc::Rc;
use std::cell::RefCell;
use std::path::PathBuf;
use std::collections::HashSet;
use anyhow::Result;
use log::{info, warn, debug, error};
use rusqlite::{Connection, params};
use directories::ProjectDirs;

// For clipboard functionality
#[cfg(target_os = "macos")]
use std::process::Command as StdCommand;

use slint::Model;

slint::include_modules!();

// ============================================================================
// Constants
// ============================================================================

/// Number of books in the Bible
#[allow(dead_code)]
const BIBLE_BOOK_COUNT_CHIRHO: usize = 66;

/// Number of Old Testament books
#[allow(dead_code)]
const OLD_TESTAMENT_BOOK_COUNT_CHIRHO: usize = 39;

/// Number of New Testament books
#[allow(dead_code)]
const NEW_TESTAMENT_BOOK_COUNT_CHIRHO: usize = 27;

/// Default module name
const DEFAULT_MODULE_CHIRHO: &str = "KJV";

/// Default book name
const DEFAULT_BOOK_CHIRHO: &str = "Genesis";

/// Default chapter number
const DEFAULT_CHAPTER_CHIRHO: i32 = 1;

/// Application identifier for directories
const APP_QUALIFIER_CHIRHO: &str = "com";
const APP_ORGANIZATION_CHIRHO: &str = "rsword";
const APP_NAME_CHIRHO: &str = "codex-lux";

/// Database filename
const DATABASE_FILENAME_CHIRHO: &str = "codex_lux_chirho.db";

/// Bible book data with chapter counts
const BIBLE_BOOKS_CHIRHO: &[(&str, i32)] = &[
    // Old Testament
    ("Genesis", 50), ("Exodus", 40), ("Leviticus", 27), ("Numbers", 36),
    ("Deuteronomy", 34), ("Joshua", 24), ("Judges", 21), ("Ruth", 4),
    ("1 Samuel", 31), ("2 Samuel", 24), ("1 Kings", 22), ("2 Kings", 25),
    ("1 Chronicles", 29), ("2 Chronicles", 36), ("Ezra", 10), ("Nehemiah", 13),
    ("Esther", 10), ("Job", 42), ("Psalms", 150), ("Proverbs", 31),
    ("Ecclesiastes", 12), ("Song of Solomon", 8), ("Isaiah", 66), ("Jeremiah", 52),
    ("Lamentations", 5), ("Ezekiel", 48), ("Daniel", 12), ("Hosea", 14),
    ("Joel", 3), ("Amos", 9), ("Obadiah", 1), ("Jonah", 4),
    ("Micah", 7), ("Nahum", 3), ("Habakkuk", 3), ("Zephaniah", 3),
    ("Haggai", 2), ("Zechariah", 14), ("Malachi", 4),
    // New Testament
    ("Matthew", 28), ("Mark", 16), ("Luke", 24), ("John", 21),
    ("Acts", 28), ("Romans", 16), ("1 Corinthians", 16), ("2 Corinthians", 13),
    ("Galatians", 6), ("Ephesians", 6), ("Philippians", 4), ("Colossians", 4),
    ("1 Thessalonians", 5), ("2 Thessalonians", 3), ("1 Timothy", 6), ("2 Timothy", 4),
    ("Titus", 3), ("Philemon", 1), ("Hebrews", 13), ("James", 5),
    ("1 Peter", 5), ("2 Peter", 3), ("1 John", 5), ("2 John", 1),
    ("3 John", 1), ("Jude", 1), ("Revelation", 22),
];

// ============================================================================
// Database Module
// ============================================================================

mod database_chirho {
    use super::*;

    /// Initialize the application database
    pub fn init_database_chirho(db_path_chirho: &PathBuf) -> Result<Connection> {
        // Create parent directories if needed
        if let Some(parent_chirho) = db_path_chirho.parent() {
            std::fs::create_dir_all(parent_chirho)?;
        }

        let conn_chirho = Connection::open(db_path_chirho)?;

        // Create tables
        conn_chirho.execute_batch(
            "
            -- Highlights table
            CREATE TABLE IF NOT EXISTS highlights_chirho (
                id_chirho INTEGER PRIMARY KEY AUTOINCREMENT,
                module_chirho TEXT NOT NULL,
                book_chirho TEXT NOT NULL,
                chapter_chirho INTEGER NOT NULL,
                verse_chirho INTEGER NOT NULL,
                color_chirho TEXT NOT NULL DEFAULT 'yellow',
                created_at_chirho DATETIME DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(module_chirho, book_chirho, chapter_chirho, verse_chirho)
            );

            -- Bookmarks table
            CREATE TABLE IF NOT EXISTS bookmarks_chirho (
                id_chirho INTEGER PRIMARY KEY AUTOINCREMENT,
                module_chirho TEXT NOT NULL,
                book_chirho TEXT NOT NULL,
                chapter_chirho INTEGER NOT NULL,
                verse_chirho INTEGER NOT NULL,
                verse_end_chirho INTEGER,
                label_chirho TEXT,
                folder_id_chirho INTEGER,
                created_at_chirho DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at_chirho DATETIME DEFAULT CURRENT_TIMESTAMP
            );

            -- Notes table
            CREATE TABLE IF NOT EXISTS notes_chirho (
                id_chirho INTEGER PRIMARY KEY AUTOINCREMENT,
                module_chirho TEXT NOT NULL,
                book_chirho TEXT NOT NULL,
                chapter_chirho INTEGER NOT NULL,
                verse_chirho INTEGER NOT NULL,
                content_chirho TEXT NOT NULL,
                created_at_chirho DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at_chirho DATETIME DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(module_chirho, book_chirho, chapter_chirho, verse_chirho)
            );

            -- Settings table
            CREATE TABLE IF NOT EXISTS settings_chirho (
                key_chirho TEXT PRIMARY KEY,
                value_chirho TEXT NOT NULL
            );

            -- Reading history table
            CREATE TABLE IF NOT EXISTS reading_history_chirho (
                id_chirho INTEGER PRIMARY KEY AUTOINCREMENT,
                module_chirho TEXT NOT NULL,
                book_chirho TEXT NOT NULL,
                chapter_chirho INTEGER NOT NULL,
                timestamp_chirho DATETIME DEFAULT CURRENT_TIMESTAMP
            );

            -- Reading sessions table for time tracking
            CREATE TABLE IF NOT EXISTS reading_sessions_chirho (
                id_chirho INTEGER PRIMARY KEY AUTOINCREMENT,
                date_chirho DATE NOT NULL,
                duration_seconds_chirho INTEGER NOT NULL DEFAULT 0,
                chapters_read_chirho INTEGER NOT NULL DEFAULT 0,
                UNIQUE(date_chirho)
            );

            -- Study journal entries table
            CREATE TABLE IF NOT EXISTS journal_entries_chirho (
                id_chirho INTEGER PRIMARY KEY AUTOINCREMENT,
                title_chirho TEXT NOT NULL,
                content_chirho TEXT NOT NULL,
                verse_ref_chirho TEXT,
                tags_chirho TEXT,
                created_at_chirho DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at_chirho DATETIME DEFAULT CURRENT_TIMESTAMP
            );

            -- Prayer requests table
            CREATE TABLE IF NOT EXISTS prayer_requests_chirho (
                id_chirho INTEGER PRIMARY KEY AUTOINCREMENT,
                title_chirho TEXT NOT NULL,
                description_chirho TEXT,
                verse_ref_chirho TEXT,
                category_chirho TEXT,
                is_answered_chirho INTEGER DEFAULT 0,
                answered_at_chirho DATETIME,
                created_at_chirho DATETIME DEFAULT CURRENT_TIMESTAMP
            );

            -- Reading plans table
            CREATE TABLE IF NOT EXISTS reading_plans_chirho (
                id_chirho INTEGER PRIMARY KEY AUTOINCREMENT,
                name_chirho TEXT NOT NULL,
                description_chirho TEXT,
                plan_type_chirho TEXT NOT NULL, -- 'daily', 'chronological', 'custom'
                total_days_chirho INTEGER NOT NULL,
                current_day_chirho INTEGER NOT NULL DEFAULT 1,
                start_date_chirho DATE,
                is_active_chirho INTEGER DEFAULT 0,
                created_at_chirho DATETIME DEFAULT CURRENT_TIMESTAMP
            );

            -- Reading plan days (individual day assignments)
            CREATE TABLE IF NOT EXISTS reading_plan_days_chirho (
                id_chirho INTEGER PRIMARY KEY AUTOINCREMENT,
                plan_id_chirho INTEGER NOT NULL,
                day_number_chirho INTEGER NOT NULL,
                readings_chirho TEXT NOT NULL, -- JSON array of verse references
                is_completed_chirho INTEGER DEFAULT 0,
                completed_at_chirho DATETIME,
                FOREIGN KEY (plan_id_chirho) REFERENCES reading_plans_chirho(id_chirho)
            );

            -- Create indexes for faster lookups
            CREATE INDEX IF NOT EXISTS idx_journal_created_chirho
                ON journal_entries_chirho(created_at_chirho);
            CREATE INDEX IF NOT EXISTS idx_highlights_location_chirho
                ON highlights_chirho(module_chirho, book_chirho, chapter_chirho);
            CREATE INDEX IF NOT EXISTS idx_notes_location_chirho
                ON notes_chirho(module_chirho, book_chirho, chapter_chirho);
            CREATE INDEX IF NOT EXISTS idx_bookmarks_location_chirho
                ON bookmarks_chirho(module_chirho, book_chirho, chapter_chirho);
            "
        )?;

        info!("Database initialized at {:?}", db_path_chirho);
        Ok(conn_chirho)
    }

    /// Get all highlights for a chapter (just verse numbers)
    #[allow(dead_code)]
    pub fn get_highlights_chirho(
        conn_chirho: &Connection,
        module_chirho: &str,
        book_chirho: &str,
        chapter_chirho: i32,
    ) -> Result<HashSet<i32>> {
        let mut stmt_chirho = conn_chirho.prepare(
            "SELECT verse_chirho FROM highlights_chirho
             WHERE module_chirho = ?1 AND book_chirho = ?2 AND chapter_chirho = ?3"
        )?;

        let verses_chirho = stmt_chirho
            .query_map(params![module_chirho, book_chirho, chapter_chirho], |row_chirho| {
                row_chirho.get::<_, i32>(0)
            })?
            .filter_map(|r_chirho| r_chirho.ok())
            .collect();

        Ok(verses_chirho)
    }

    /// Toggle a verse highlight
    pub fn toggle_highlight_chirho(
        conn_chirho: &Connection,
        module_chirho: &str,
        book_chirho: &str,
        chapter_chirho: i32,
        verse_chirho: i32,
    ) -> Result<bool> {
        // Check if highlight exists
        let exists_chirho: bool = conn_chirho.query_row(
            "SELECT 1 FROM highlights_chirho
             WHERE module_chirho = ?1 AND book_chirho = ?2 AND chapter_chirho = ?3 AND verse_chirho = ?4",
            params![module_chirho, book_chirho, chapter_chirho, verse_chirho],
            |_| Ok(true)
        ).unwrap_or(false);

        if exists_chirho {
            // Remove highlight
            conn_chirho.execute(
                "DELETE FROM highlights_chirho
                 WHERE module_chirho = ?1 AND book_chirho = ?2 AND chapter_chirho = ?3 AND verse_chirho = ?4",
                params![module_chirho, book_chirho, chapter_chirho, verse_chirho],
            )?;
            Ok(false)
        } else {
            // Add highlight
            conn_chirho.execute(
                "INSERT INTO highlights_chirho (module_chirho, book_chirho, chapter_chirho, verse_chirho)
                 VALUES (?1, ?2, ?3, ?4)",
                params![module_chirho, book_chirho, chapter_chirho, verse_chirho],
            )?;
            Ok(true)
        }
    }

    /// Check if a verse has a note
    pub fn get_verses_with_notes_chirho(
        conn_chirho: &Connection,
        module_chirho: &str,
        book_chirho: &str,
        chapter_chirho: i32,
    ) -> Result<HashSet<i32>> {
        let mut stmt_chirho = conn_chirho.prepare(
            "SELECT verse_chirho FROM notes_chirho
             WHERE module_chirho = ?1 AND book_chirho = ?2 AND chapter_chirho = ?3"
        )?;

        let verses_chirho = stmt_chirho
            .query_map(params![module_chirho, book_chirho, chapter_chirho], |row_chirho| {
                row_chirho.get::<_, i32>(0)
            })?
            .filter_map(|r_chirho| r_chirho.ok())
            .collect();

        Ok(verses_chirho)
    }

    /// Save a setting
    pub fn set_setting_chirho(conn_chirho: &Connection, key_chirho: &str, value_chirho: &str) -> Result<()> {
        conn_chirho.execute(
            "INSERT OR REPLACE INTO settings_chirho (key_chirho, value_chirho) VALUES (?1, ?2)",
            params![key_chirho, value_chirho],
        )?;
        Ok(())
    }

    /// Get a setting
    pub fn get_setting_chirho(conn_chirho: &Connection, key_chirho: &str) -> Option<String> {
        conn_chirho.query_row(
            "SELECT value_chirho FROM settings_chirho WHERE key_chirho = ?1",
            params![key_chirho],
            |row_chirho| row_chirho.get(0)
        ).ok()
    }

    /// Add to reading history
    pub fn add_reading_history_chirho(
        conn_chirho: &Connection,
        module_chirho: &str,
        book_chirho: &str,
        chapter_chirho: i32,
    ) -> Result<()> {
        conn_chirho.execute(
            "INSERT INTO reading_history_chirho (module_chirho, book_chirho, chapter_chirho)
             VALUES (?1, ?2, ?3)",
            params![module_chirho, book_chirho, chapter_chirho],
        )?;
        Ok(())
    }

    /// Reading history entry
    #[derive(Debug, Clone)]
    pub struct HistoryEntryChirho {
        #[allow(dead_code)]
        pub id_chirho: i64,
        pub book_chirho: String,
        pub chapter_chirho: i32,
        pub timestamp_chirho: String,
    }

    /// Get recent reading history (distinct locations, most recent first)
    pub fn get_recent_history_chirho(conn_chirho: &Connection, limit_chirho: usize) -> Result<Vec<HistoryEntryChirho>> {
        let mut stmt_chirho = conn_chirho.prepare(
            "SELECT id_chirho, book_chirho, chapter_chirho, timestamp_chirho
             FROM reading_history_chirho
             GROUP BY book_chirho, chapter_chirho
             ORDER BY MAX(timestamp_chirho) DESC
             LIMIT ?1"
        )?;

        let entries_chirho = stmt_chirho.query_map([limit_chirho], |row_chirho| {
            Ok(HistoryEntryChirho {
                id_chirho: row_chirho.get(0)?,
                book_chirho: row_chirho.get(1)?,
                chapter_chirho: row_chirho.get(2)?,
                timestamp_chirho: row_chirho.get(3)?,
            })
        })?;

        Ok(entries_chirho.filter_map(|r_chirho| r_chirho.ok()).collect())
    }

    /// Highlight data structure
    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    pub struct HighlightChirho {
        pub id_chirho: i64,
        pub module_chirho: String,
        pub book_chirho: String,
        pub chapter_chirho: i32,
        pub verse_chirho: i32,
        pub color_chirho: String,
    }

    /// Get all highlights
    pub fn get_all_highlights_chirho(conn_chirho: &Connection) -> Result<Vec<HighlightChirho>> {
        let mut stmt_chirho = conn_chirho.prepare(
            "SELECT id_chirho, module_chirho, book_chirho, chapter_chirho, verse_chirho, color_chirho
             FROM highlights_chirho ORDER BY created_at_chirho DESC"
        )?;

        let highlights_chirho = stmt_chirho
            .query_map([], |row_chirho| {
                Ok(HighlightChirho {
                    id_chirho: row_chirho.get(0)?,
                    module_chirho: row_chirho.get(1)?,
                    book_chirho: row_chirho.get(2)?,
                    chapter_chirho: row_chirho.get(3)?,
                    verse_chirho: row_chirho.get(4)?,
                    color_chirho: row_chirho.get(5)?,
                })
            })?
            .filter_map(|r_chirho| r_chirho.ok())
            .collect();

        Ok(highlights_chirho)
    }

    /// Bookmark data structure
    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    pub struct BookmarkChirho {
        pub id_chirho: i64,
        pub module_chirho: String,
        pub book_chirho: String,
        pub chapter_chirho: i32,
        pub verse_chirho: i32,
        pub label_chirho: Option<String>,
    }

    /// Add a bookmark
    #[allow(dead_code)]
    pub fn add_bookmark_chirho(
        conn_chirho: &Connection,
        module_chirho: &str,
        book_chirho: &str,
        chapter_chirho: i32,
        verse_chirho: i32,
        label_chirho: Option<&str>,
    ) -> Result<i64> {
        conn_chirho.execute(
            "INSERT INTO bookmarks_chirho (module_chirho, book_chirho, chapter_chirho, verse_chirho, label_chirho)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![module_chirho, book_chirho, chapter_chirho, verse_chirho, label_chirho],
        )?;
        Ok(conn_chirho.last_insert_rowid())
    }

    /// Remove a bookmark
    #[allow(dead_code)]
    pub fn remove_bookmark_chirho(conn_chirho: &Connection, id_chirho: i64) -> Result<()> {
        conn_chirho.execute(
            "DELETE FROM bookmarks_chirho WHERE id_chirho = ?1",
            params![id_chirho],
        )?;
        Ok(())
    }

    /// Update bookmark label
    pub fn update_bookmark_label_chirho(conn_chirho: &Connection, id_chirho: i64, label_chirho: &str) -> Result<()> {
        conn_chirho.execute(
            "UPDATE bookmarks_chirho SET label_chirho = ?1, updated_at_chirho = CURRENT_TIMESTAMP WHERE id_chirho = ?2",
            params![label_chirho, id_chirho],
        )?;
        Ok(())
    }

    /// Get all bookmarks
    #[allow(dead_code)]
    pub fn get_all_bookmarks_chirho(conn_chirho: &Connection) -> Result<Vec<BookmarkChirho>> {
        let mut stmt_chirho = conn_chirho.prepare(
            "SELECT id_chirho, module_chirho, book_chirho, chapter_chirho, verse_chirho, label_chirho
             FROM bookmarks_chirho ORDER BY created_at_chirho DESC"
        )?;

        let bookmarks_chirho = stmt_chirho
            .query_map([], |row_chirho| {
                Ok(BookmarkChirho {
                    id_chirho: row_chirho.get(0)?,
                    module_chirho: row_chirho.get(1)?,
                    book_chirho: row_chirho.get(2)?,
                    chapter_chirho: row_chirho.get(3)?,
                    verse_chirho: row_chirho.get(4)?,
                    label_chirho: row_chirho.get(5)?,
                })
            })?
            .filter_map(|r_chirho| r_chirho.ok())
            .collect();

        Ok(bookmarks_chirho)
    }

    /// Check if a verse is bookmarked
    #[allow(dead_code)]
    pub fn is_bookmarked_chirho(
        conn_chirho: &Connection,
        module_chirho: &str,
        book_chirho: &str,
        chapter_chirho: i32,
        verse_chirho: i32,
    ) -> bool {
        conn_chirho.query_row(
            "SELECT 1 FROM bookmarks_chirho
             WHERE module_chirho = ?1 AND book_chirho = ?2 AND chapter_chirho = ?3 AND verse_chirho = ?4",
            params![module_chirho, book_chirho, chapter_chirho, verse_chirho],
            |_| Ok(true)
        ).unwrap_or(false)
    }

    /// Get bookmark ID for a verse (if bookmarked)
    #[allow(dead_code)]
    pub fn get_bookmark_id_chirho(
        conn_chirho: &Connection,
        module_chirho: &str,
        book_chirho: &str,
        chapter_chirho: i32,
        verse_chirho: i32,
    ) -> Option<i64> {
        conn_chirho.query_row(
            "SELECT id_chirho FROM bookmarks_chirho
             WHERE module_chirho = ?1 AND book_chirho = ?2 AND chapter_chirho = ?3 AND verse_chirho = ?4",
            params![module_chirho, book_chirho, chapter_chirho, verse_chirho],
            |row_chirho| row_chirho.get(0)
        ).ok()
    }

    /// Note data structure
    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    pub struct NoteChirho {
        pub id_chirho: i64,
        pub module_chirho: String,
        pub book_chirho: String,
        pub chapter_chirho: i32,
        pub verse_chirho: i32,
        pub content_chirho: String,
        pub created_at_chirho: String,
        pub updated_at_chirho: String,
    }

    /// Save or update a note
    #[allow(dead_code)]
    pub fn save_note_chirho(
        conn_chirho: &Connection,
        module_chirho: &str,
        book_chirho: &str,
        chapter_chirho: i32,
        verse_chirho: i32,
        content_chirho: &str,
    ) -> Result<i64> {
        conn_chirho.execute(
            "INSERT INTO notes_chirho (module_chirho, book_chirho, chapter_chirho, verse_chirho, content_chirho)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(module_chirho, book_chirho, chapter_chirho, verse_chirho)
             DO UPDATE SET content_chirho = excluded.content_chirho, updated_at_chirho = CURRENT_TIMESTAMP",
            params![module_chirho, book_chirho, chapter_chirho, verse_chirho, content_chirho],
        )?;
        Ok(conn_chirho.last_insert_rowid())
    }

    /// Get a note for a specific verse
    #[allow(dead_code)]
    pub fn get_note_chirho(
        conn_chirho: &Connection,
        module_chirho: &str,
        book_chirho: &str,
        chapter_chirho: i32,
        verse_chirho: i32,
    ) -> Option<NoteChirho> {
        conn_chirho.query_row(
            "SELECT id_chirho, module_chirho, book_chirho, chapter_chirho, verse_chirho, content_chirho,
                    created_at_chirho, updated_at_chirho
             FROM notes_chirho
             WHERE module_chirho = ?1 AND book_chirho = ?2 AND chapter_chirho = ?3 AND verse_chirho = ?4",
            params![module_chirho, book_chirho, chapter_chirho, verse_chirho],
            |row_chirho| {
                Ok(NoteChirho {
                    id_chirho: row_chirho.get(0)?,
                    module_chirho: row_chirho.get(1)?,
                    book_chirho: row_chirho.get(2)?,
                    chapter_chirho: row_chirho.get(3)?,
                    verse_chirho: row_chirho.get(4)?,
                    content_chirho: row_chirho.get(5)?,
                    created_at_chirho: row_chirho.get(6)?,
                    updated_at_chirho: row_chirho.get(7)?,
                })
            }
        ).ok()
    }

    /// Delete a note
    #[allow(dead_code)]
    pub fn delete_note_chirho(
        conn_chirho: &Connection,
        module_chirho: &str,
        book_chirho: &str,
        chapter_chirho: i32,
        verse_chirho: i32,
    ) -> Result<()> {
        conn_chirho.execute(
            "DELETE FROM notes_chirho
             WHERE module_chirho = ?1 AND book_chirho = ?2 AND chapter_chirho = ?3 AND verse_chirho = ?4",
            params![module_chirho, book_chirho, chapter_chirho, verse_chirho],
        )?;
        Ok(())
    }

    /// Get all notes
    #[allow(dead_code)]
    pub fn get_all_notes_chirho(conn_chirho: &Connection) -> Result<Vec<NoteChirho>> {
        let mut stmt_chirho = conn_chirho.prepare(
            "SELECT id_chirho, module_chirho, book_chirho, chapter_chirho, verse_chirho, content_chirho,
                    created_at_chirho, updated_at_chirho
             FROM notes_chirho ORDER BY updated_at_chirho DESC"
        )?;

        let notes_chirho = stmt_chirho
            .query_map([], |row_chirho| {
                Ok(NoteChirho {
                    id_chirho: row_chirho.get(0)?,
                    module_chirho: row_chirho.get(1)?,
                    book_chirho: row_chirho.get(2)?,
                    chapter_chirho: row_chirho.get(3)?,
                    verse_chirho: row_chirho.get(4)?,
                    content_chirho: row_chirho.get(5)?,
                    created_at_chirho: row_chirho.get(6)?,
                    updated_at_chirho: row_chirho.get(7)?,
                })
            })?
            .filter_map(|r_chirho| r_chirho.ok())
            .collect();

        Ok(notes_chirho)
    }

    /// Search notes by content
    #[allow(dead_code)]
    pub fn search_notes_chirho(conn_chirho: &Connection, query_chirho: &str) -> Result<Vec<NoteChirho>> {
        let search_pattern_chirho = format!("%{}%", query_chirho);
        let mut stmt_chirho = conn_chirho.prepare(
            "SELECT id_chirho, module_chirho, book_chirho, chapter_chirho, verse_chirho, content_chirho,
                    created_at_chirho, updated_at_chirho
             FROM notes_chirho
             WHERE content_chirho LIKE ?1
             ORDER BY updated_at_chirho DESC"
        )?;

        let notes_chirho = stmt_chirho
            .query_map(params![search_pattern_chirho], |row_chirho| {
                Ok(NoteChirho {
                    id_chirho: row_chirho.get(0)?,
                    module_chirho: row_chirho.get(1)?,
                    book_chirho: row_chirho.get(2)?,
                    chapter_chirho: row_chirho.get(3)?,
                    verse_chirho: row_chirho.get(4)?,
                    content_chirho: row_chirho.get(5)?,
                    created_at_chirho: row_chirho.get(6)?,
                    updated_at_chirho: row_chirho.get(7)?,
                })
            })?
            .filter_map(|r_chirho| r_chirho.ok())
            .collect();

        Ok(notes_chirho)
    }

    /// Highlight color data
    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    pub struct HighlightInfoChirho {
        pub verse_chirho: i32,
        pub color_chirho: String,
    }

    /// Get highlights with color information
    #[allow(dead_code)]
    pub fn get_highlights_with_colors_chirho(
        conn_chirho: &Connection,
        module_chirho: &str,
        book_chirho: &str,
        chapter_chirho: i32,
    ) -> Result<Vec<HighlightInfoChirho>> {
        let mut stmt_chirho = conn_chirho.prepare(
            "SELECT verse_chirho, color_chirho FROM highlights_chirho
             WHERE module_chirho = ?1 AND book_chirho = ?2 AND chapter_chirho = ?3"
        )?;

        let highlights_chirho = stmt_chirho
            .query_map(params![module_chirho, book_chirho, chapter_chirho], |row_chirho| {
                Ok(HighlightInfoChirho {
                    verse_chirho: row_chirho.get(0)?,
                    color_chirho: row_chirho.get(1)?,
                })
            })?
            .filter_map(|r_chirho| r_chirho.ok())
            .collect();

        Ok(highlights_chirho)
    }

    /// Add highlight with specific color
    #[allow(dead_code)]
    pub fn add_highlight_with_color_chirho(
        conn_chirho: &Connection,
        module_chirho: &str,
        book_chirho: &str,
        chapter_chirho: i32,
        verse_chirho: i32,
        color_chirho: &str,
    ) -> Result<()> {
        conn_chirho.execute(
            "INSERT OR REPLACE INTO highlights_chirho (module_chirho, book_chirho, chapter_chirho, verse_chirho, color_chirho)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![module_chirho, book_chirho, chapter_chirho, verse_chirho, color_chirho],
        )?;
        Ok(())
    }

    // ========================================================================
    // Statistics Functions
    // ========================================================================

    /// Statistics data structure
    #[derive(Debug)]
    #[allow(dead_code)]
    pub struct StatisticsChirho {
        pub total_chapters_read_chirho: i32,
        pub unique_chapters_read_chirho: i32,
        pub total_reading_time_seconds_chirho: i64,
        pub current_streak_chirho: i32,
        pub longest_streak_chirho: i32,
        pub highlight_count_chirho: i32,
        pub note_count_chirho: i32,
        pub bookmark_count_chirho: i32,
        pub books_started_chirho: i32,
        pub books_completed_chirho: i32,
    }

    /// Record a reading session for today (will be used for time tracking feature)
    #[allow(dead_code)]
    pub fn record_reading_session_chirho(
        conn_chirho: &Connection,
        duration_seconds_chirho: i64,
        chapters_read_chirho: i32,
    ) -> Result<()> {
        let today_chirho = chrono::Local::now().format("%Y-%m-%d").to_string();
        conn_chirho.execute(
            "INSERT INTO reading_sessions_chirho (date_chirho, duration_seconds_chirho, chapters_read_chirho)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(date_chirho) DO UPDATE SET
                 duration_seconds_chirho = duration_seconds_chirho + excluded.duration_seconds_chirho,
                 chapters_read_chirho = chapters_read_chirho + excluded.chapters_read_chirho",
            params![today_chirho, duration_seconds_chirho, chapters_read_chirho],
        )?;
        Ok(())
    }

    /// Get complete reading statistics
    pub fn get_statistics_chirho(conn_chirho: &Connection) -> Result<StatisticsChirho> {
        // Total and unique chapters read from reading_history
        let total_chapters_read_chirho: i32 = conn_chirho.query_row(
            "SELECT COUNT(*) FROM reading_history_chirho",
            [],
            |row_chirho| row_chirho.get(0)
        ).unwrap_or(0);

        let unique_chapters_read_chirho: i32 = conn_chirho.query_row(
            "SELECT COUNT(DISTINCT book_chirho || ':' || chapter_chirho) FROM reading_history_chirho",
            [],
            |row_chirho| row_chirho.get(0)
        ).unwrap_or(0);

        // Total reading time
        let total_reading_time_seconds_chirho: i64 = conn_chirho.query_row(
            "SELECT COALESCE(SUM(duration_seconds_chirho), 0) FROM reading_sessions_chirho",
            [],
            |row_chirho| row_chirho.get(0)
        ).unwrap_or(0);

        // Calculate reading streaks
        let (current_streak_chirho, longest_streak_chirho) = calculate_reading_streaks_chirho(conn_chirho);

        // Count highlights
        let highlight_count_chirho: i32 = conn_chirho.query_row(
            "SELECT COUNT(*) FROM highlights_chirho",
            [],
            |row_chirho| row_chirho.get(0)
        ).unwrap_or(0);

        // Count notes
        let note_count_chirho: i32 = conn_chirho.query_row(
            "SELECT COUNT(*) FROM notes_chirho",
            [],
            |row_chirho| row_chirho.get(0)
        ).unwrap_or(0);

        // Count bookmarks
        let bookmark_count_chirho: i32 = conn_chirho.query_row(
            "SELECT COUNT(*) FROM bookmarks_chirho",
            [],
            |row_chirho| row_chirho.get(0)
        ).unwrap_or(0);

        // Books started (have at least 1 chapter read)
        let books_started_chirho: i32 = conn_chirho.query_row(
            "SELECT COUNT(DISTINCT book_chirho) FROM reading_history_chirho",
            [],
            |row_chirho| row_chirho.get(0)
        ).unwrap_or(0);

        // Books completed (all chapters of a book read)
        // This is approximate - we'd need book chapter counts
        let books_completed_chirho: i32 = 0; // TODO: Implement with proper chapter count data

        Ok(StatisticsChirho {
            total_chapters_read_chirho,
            unique_chapters_read_chirho,
            total_reading_time_seconds_chirho,
            current_streak_chirho,
            longest_streak_chirho,
            highlight_count_chirho,
            note_count_chirho,
            bookmark_count_chirho,
            books_started_chirho,
            books_completed_chirho,
        })
    }

    /// Calculate current and longest reading streaks
    fn calculate_reading_streaks_chirho(conn_chirho: &Connection) -> (i32, i32) {
        // Get all reading session dates
        let mut stmt_chirho = match conn_chirho.prepare(
            "SELECT DISTINCT date_chirho FROM reading_sessions_chirho ORDER BY date_chirho DESC"
        ) {
            Ok(stmt_chirho) => stmt_chirho,
            Err(_) => return (0, 0),
        };

        let dates_chirho: Vec<String> = stmt_chirho
            .query_map([], |row_chirho| row_chirho.get(0))
            .map(|rows_chirho| rows_chirho.filter_map(|r_chirho| r_chirho.ok()).collect())
            .unwrap_or_default();

        if dates_chirho.is_empty() {
            return (0, 0);
        }

        let today_chirho = chrono::Local::now().format("%Y-%m-%d").to_string();
        let yesterday_chirho = (chrono::Local::now() - chrono::Duration::days(1))
            .format("%Y-%m-%d")
            .to_string();

        let mut current_streak_chirho = 0;
        let mut longest_streak_chirho = 0;
        let mut temp_streak_chirho = 0;
        let mut prev_date_opt_chirho: Option<chrono::NaiveDate> = None;

        for date_str_chirho in dates_chirho.iter() {
            if let Ok(date_chirho) = chrono::NaiveDate::parse_from_str(date_str_chirho, "%Y-%m-%d") {
                if let Some(prev_date_chirho) = prev_date_opt_chirho {
                    let diff_chirho = (prev_date_chirho - date_chirho).num_days();
                    if diff_chirho == 1 {
                        temp_streak_chirho += 1;
                    } else {
                        longest_streak_chirho = longest_streak_chirho.max(temp_streak_chirho);
                        temp_streak_chirho = 1;
                    }
                } else {
                    temp_streak_chirho = 1;
                    // Check if current streak is active (today or yesterday)
                    if date_str_chirho == &today_chirho || date_str_chirho == &yesterday_chirho {
                        current_streak_chirho = 1;
                    }
                }
                prev_date_opt_chirho = Some(date_chirho);
            }
        }
        longest_streak_chirho = longest_streak_chirho.max(temp_streak_chirho);

        // Update current streak by counting consecutive days from today/yesterday
        if dates_chirho.first() == Some(&today_chirho) || dates_chirho.first() == Some(&yesterday_chirho) {
            current_streak_chirho = 1;
            let mut prev_date_chirho = if dates_chirho.first() == Some(&today_chirho) {
                chrono::Local::now().date_naive()
            } else {
                chrono::Local::now().date_naive() - chrono::Duration::days(1)
            };

            for date_str_chirho in dates_chirho.iter().skip(1) {
                if let Ok(date_chirho) = chrono::NaiveDate::parse_from_str(date_str_chirho, "%Y-%m-%d") {
                    if (prev_date_chirho - date_chirho).num_days() == 1 {
                        current_streak_chirho += 1;
                        prev_date_chirho = date_chirho;
                    } else {
                        break;
                    }
                }
            }
        }

        (current_streak_chirho, longest_streak_chirho)
    }

    /// Format duration as human-readable string
    pub fn format_duration_chirho(seconds_chirho: i64) -> String {
        let hours_chirho = seconds_chirho / 3600;
        let minutes_chirho = (seconds_chirho % 3600) / 60;

        if hours_chirho > 0 {
            format!("{}h {}m", hours_chirho, minutes_chirho)
        } else {
            format!("{}m", minutes_chirho)
        }
    }

    // ========================================================================
    // Journal Functions
    // ========================================================================

    /// Journal entry data structure
    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    pub struct JournalEntryChirho {
        pub id_chirho: i64,
        pub title_chirho: String,
        pub content_chirho: String,
        pub verse_ref_chirho: Option<String>,
        pub tags_chirho: Option<String>,
        pub created_at_chirho: String,
        pub updated_at_chirho: String,
    }

    /// Create a new journal entry
    pub fn create_journal_entry_chirho(
        conn_chirho: &Connection,
        title_chirho: &str,
        content_chirho: &str,
        verse_ref_chirho: Option<&str>,
        tags_chirho: Option<&str>,
    ) -> Result<i64> {
        conn_chirho.execute(
            "INSERT INTO journal_entries_chirho (title_chirho, content_chirho, verse_ref_chirho, tags_chirho)
             VALUES (?1, ?2, ?3, ?4)",
            params![title_chirho, content_chirho, verse_ref_chirho, tags_chirho],
        )?;
        Ok(conn_chirho.last_insert_rowid())
    }

    /// Update a journal entry
    pub fn update_journal_entry_chirho(
        conn_chirho: &Connection,
        id_chirho: i64,
        title_chirho: &str,
        content_chirho: &str,
        verse_ref_chirho: Option<&str>,
        tags_chirho: Option<&str>,
    ) -> Result<()> {
        conn_chirho.execute(
            "UPDATE journal_entries_chirho
             SET title_chirho = ?1, content_chirho = ?2, verse_ref_chirho = ?3, tags_chirho = ?4, updated_at_chirho = CURRENT_TIMESTAMP
             WHERE id_chirho = ?5",
            params![title_chirho, content_chirho, verse_ref_chirho, tags_chirho, id_chirho],
        )?;
        Ok(())
    }

    /// Delete a journal entry
    pub fn delete_journal_entry_chirho(conn_chirho: &Connection, id_chirho: i64) -> Result<()> {
        conn_chirho.execute(
            "DELETE FROM journal_entries_chirho WHERE id_chirho = ?1",
            params![id_chirho],
        )?;
        Ok(())
    }

    /// Get a journal entry by ID (for loading full content in editor)
    #[allow(dead_code)]
    pub fn get_journal_entry_chirho(conn_chirho: &Connection, id_chirho: i64) -> Option<JournalEntryChirho> {
        conn_chirho.query_row(
            "SELECT id_chirho, title_chirho, content_chirho, verse_ref_chirho, tags_chirho, created_at_chirho, updated_at_chirho
             FROM journal_entries_chirho WHERE id_chirho = ?1",
            params![id_chirho],
            |row_chirho| Ok(JournalEntryChirho {
                id_chirho: row_chirho.get(0)?,
                title_chirho: row_chirho.get(1)?,
                content_chirho: row_chirho.get(2)?,
                verse_ref_chirho: row_chirho.get(3)?,
                tags_chirho: row_chirho.get(4)?,
                created_at_chirho: row_chirho.get(5)?,
                updated_at_chirho: row_chirho.get(6)?,
            })
        ).ok()
    }

    /// Get all journal entries, ordered by most recent
    pub fn get_all_journal_entries_chirho(conn_chirho: &Connection) -> Result<Vec<JournalEntryChirho>> {
        let mut stmt_chirho = conn_chirho.prepare(
            "SELECT id_chirho, title_chirho, content_chirho, verse_ref_chirho, tags_chirho, created_at_chirho, updated_at_chirho
             FROM journal_entries_chirho ORDER BY created_at_chirho DESC"
        )?;

        let entries_chirho = stmt_chirho
            .query_map([], |row_chirho| Ok(JournalEntryChirho {
                id_chirho: row_chirho.get(0)?,
                title_chirho: row_chirho.get(1)?,
                content_chirho: row_chirho.get(2)?,
                verse_ref_chirho: row_chirho.get(3)?,
                tags_chirho: row_chirho.get(4)?,
                created_at_chirho: row_chirho.get(5)?,
                updated_at_chirho: row_chirho.get(6)?,
            }))?
            .filter_map(|r_chirho| r_chirho.ok())
            .collect();

        Ok(entries_chirho)
    }

    /// Search journal entries (for journal search feature)
    #[allow(dead_code)]
    pub fn search_journal_entries_chirho(conn_chirho: &Connection, query_chirho: &str) -> Result<Vec<JournalEntryChirho>> {
        let search_pattern_chirho = format!("%{}%", query_chirho);
        let mut stmt_chirho = conn_chirho.prepare(
            "SELECT id_chirho, title_chirho, content_chirho, verse_ref_chirho, tags_chirho, created_at_chirho, updated_at_chirho
             FROM journal_entries_chirho
             WHERE title_chirho LIKE ?1 OR content_chirho LIKE ?1 OR tags_chirho LIKE ?1
             ORDER BY created_at_chirho DESC"
        )?;

        let entries_chirho = stmt_chirho
            .query_map([&search_pattern_chirho], |row_chirho| Ok(JournalEntryChirho {
                id_chirho: row_chirho.get(0)?,
                title_chirho: row_chirho.get(1)?,
                content_chirho: row_chirho.get(2)?,
                verse_ref_chirho: row_chirho.get(3)?,
                tags_chirho: row_chirho.get(4)?,
                created_at_chirho: row_chirho.get(5)?,
                updated_at_chirho: row_chirho.get(6)?,
            }))?
            .filter_map(|r_chirho| r_chirho.ok())
            .collect();

        Ok(entries_chirho)
    }

    /// Get journal entry count (for statistics)
    #[allow(dead_code)]
    pub fn get_journal_entry_count_chirho(conn_chirho: &Connection) -> i32 {
        conn_chirho.query_row(
            "SELECT COUNT(*) FROM journal_entries_chirho",
            [],
            |row_chirho| row_chirho.get(0)
        ).unwrap_or(0)
    }

    // ========================================================================
    // Prayer Request Functions
    // ========================================================================

    /// Prayer request data structure
    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    pub struct PrayerRequestChirho {
        pub id_chirho: i64,
        pub title_chirho: String,
        pub description_chirho: Option<String>,
        pub verse_ref_chirho: Option<String>,
        pub category_chirho: Option<String>,
        pub is_answered_chirho: bool,
        pub answered_at_chirho: Option<String>,
        pub created_at_chirho: String,
    }

    /// Create a new prayer request
    pub fn create_prayer_request_chirho(
        conn_chirho: &Connection,
        title_chirho: &str,
        description_chirho: Option<&str>,
        verse_ref_chirho: Option<&str>,
        category_chirho: Option<&str>,
    ) -> Result<i64> {
        conn_chirho.execute(
            "INSERT INTO prayer_requests_chirho (title_chirho, description_chirho, verse_ref_chirho, category_chirho)
             VALUES (?1, ?2, ?3, ?4)",
            params![title_chirho, description_chirho, verse_ref_chirho, category_chirho],
        )?;
        Ok(conn_chirho.last_insert_rowid())
    }

    /// Mark a prayer request as answered
    pub fn mark_prayer_answered_chirho(conn_chirho: &Connection, id_chirho: i64) -> Result<()> {
        conn_chirho.execute(
            "UPDATE prayer_requests_chirho
             SET is_answered_chirho = 1, answered_at_chirho = CURRENT_TIMESTAMP
             WHERE id_chirho = ?1",
            params![id_chirho],
        )?;
        Ok(())
    }

    /// Delete a prayer request
    pub fn delete_prayer_request_chirho(conn_chirho: &Connection, id_chirho: i64) -> Result<()> {
        conn_chirho.execute(
            "DELETE FROM prayer_requests_chirho WHERE id_chirho = ?1",
            params![id_chirho],
        )?;
        Ok(())
    }

    /// Get all prayer requests (active first, then answered)
    pub fn get_all_prayer_requests_chirho(conn_chirho: &Connection) -> Result<Vec<PrayerRequestChirho>> {
        let mut stmt_chirho = conn_chirho.prepare(
            "SELECT id_chirho, title_chirho, description_chirho, verse_ref_chirho, category_chirho,
                    is_answered_chirho, answered_at_chirho, created_at_chirho
             FROM prayer_requests_chirho
             ORDER BY is_answered_chirho ASC, created_at_chirho DESC"
        )?;

        let requests_chirho = stmt_chirho
            .query_map([], |row_chirho| Ok(PrayerRequestChirho {
                id_chirho: row_chirho.get(0)?,
                title_chirho: row_chirho.get(1)?,
                description_chirho: row_chirho.get(2)?,
                verse_ref_chirho: row_chirho.get(3)?,
                category_chirho: row_chirho.get(4)?,
                is_answered_chirho: row_chirho.get::<_, i32>(5)? != 0,
                answered_at_chirho: row_chirho.get(6)?,
                created_at_chirho: row_chirho.get(7)?,
            }))?
            .filter_map(|r_chirho| r_chirho.ok())
            .collect();

        Ok(requests_chirho)
    }

    /// Get prayer request count (active and total)
    #[allow(dead_code)]
    pub fn get_prayer_counts_chirho(conn_chirho: &Connection) -> (i32, i32) {
        let active_chirho: i32 = conn_chirho.query_row(
            "SELECT COUNT(*) FROM prayer_requests_chirho WHERE is_answered_chirho = 0",
            [],
            |row_chirho| row_chirho.get(0)
        ).unwrap_or(0);

        let total_chirho: i32 = conn_chirho.query_row(
            "SELECT COUNT(*) FROM prayer_requests_chirho",
            [],
            |row_chirho| row_chirho.get(0)
        ).unwrap_or(0);

        (active_chirho, total_chirho)
    }
}

// ============================================================================
// Bible Engine Module
// ============================================================================

mod bible_engine_chirho {
    use super::*;
    use rsword_chirho::{SwMgrChirho, FilterChirho, OsisToPlainFilterChirho};
    use regex::Regex;
    use std::sync::LazyLock;

    // Regex patterns for extracting interlinear data from OSIS
    static OSIS_WORD_REGEX_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#"<w\s+([^>]*)>([^<]*)</w>"#).unwrap()
    });

    static OSIS_GLOSS_REGEX_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#"gloss="([^"]*)""#).unwrap()
    });

    #[allow(dead_code)]
    static OSIS_LEMMA_REGEX_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#"lemma="([^"]*)""#).unwrap()
    });

    static OSIS_STRONGS_REGEX_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#"strong:([GH]\d+)"#).unwrap()
    });

    static OSIS_MORPH_REGEX_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#"morph="([^"]*)""#).unwrap()
    });

    /// Bible engine wrapping rsword_chirho
    pub struct BibleEngineChirho {
        manager_chirho: Option<SwMgrChirho>,
        current_module_chirho: String,
        osis_filter_chirho: OsisToPlainFilterChirho,
    }

    impl BibleEngineChirho {
        /// Create a new Bible engine
        pub fn new_chirho() -> Self {
            // Try to load SWORD modules from system paths
            let manager_chirho = match SwMgrChirho::with_system_paths_chirho() {
                Ok(mgr_chirho) => {
                    let count_chirho = mgr_chirho.get_module_names_chirho().len();
                    info!("Loaded {} SWORD modules", count_chirho);
                    Some(mgr_chirho)
                }
                Err(e_chirho) => {
                    warn!("Could not load SWORD modules: {}. Using sample data.", e_chirho);
                    None
                }
            };

            Self {
                manager_chirho,
                current_module_chirho: DEFAULT_MODULE_CHIRHO.to_string(),
                osis_filter_chirho: OsisToPlainFilterChirho::new_chirho(),
            }
        }

        /// Get list of available module names
        pub fn get_module_names_chirho(&self) -> Vec<String> {
            match &self.manager_chirho {
                Some(mgr_chirho) => mgr_chirho.get_module_names_chirho()
                    .into_iter()
                    .map(|s_chirho| s_chirho.to_string())
                    .collect(),
                None => vec![
                    DEFAULT_MODULE_CHIRHO.to_string(),
                    "SBLGNT".to_string(),
                    "WLC".to_string(),
                ],
            }
        }

        /// Get verses for a chapter
        pub fn get_chapter_verses_chirho(
            &self,
            book_chirho: &str,
            chapter_chirho: i32,
        ) -> Vec<(String, String)> {
            // Try to get from SWORD module first
            if let Some(mgr_chirho) = &self.manager_chirho {
                if let Ok(loaded_module_chirho) = mgr_chirho.load_module_chirho(&self.current_module_chirho) {
                    let mut verses_chirho = Vec::new();

                    // Safety limit for verse iteration
                    let max_verse_chirho = 200;

                    for verse_num_chirho in 1..=max_verse_chirho {
                        let ref_str_chirho = format!("{} {}:{}", book_chirho, chapter_chirho, verse_num_chirho);

                        match loaded_module_chirho.read_entry_chirho(&ref_str_chirho) {
                            Ok(text_chirho) if !text_chirho.trim().is_empty() => {
                                // Apply OSIS filter to strip markup
                                let filtered_text_chirho = self.osis_filter_chirho
                                    .process_chirho(&text_chirho)
                                    .unwrap_or_else(|_| text_chirho.clone());
                                verses_chirho.push((
                                    verse_num_chirho.to_string(),
                                    filtered_text_chirho,
                                ));
                            }
                            _ => {
                                // No more verses in this chapter
                                if verse_num_chirho > 1 {
                                    break;
                                }
                            }
                        }
                    }

                    if !verses_chirho.is_empty() {
                        return verses_chirho;
                    }
                }
            }

            // Fallback to sample verses
            get_sample_verses_chirho(book_chirho, chapter_chirho)
        }

        /// Get verses for a chapter from a specific module (for parallel view)
        pub fn get_chapter_verses_for_module_chirho(
            &self,
            module_name_chirho: &str,
            book_chirho: &str,
            chapter_chirho: i32,
        ) -> Vec<(String, String)> {
            // Try to get from SWORD module
            if let Some(mgr_chirho) = &self.manager_chirho {
                if let Ok(loaded_module_chirho) = mgr_chirho.load_module_chirho(module_name_chirho) {
                    let mut verses_chirho = Vec::new();
                    let max_verse_chirho = 200;

                    for verse_num_chirho in 1..=max_verse_chirho {
                        let ref_str_chirho = format!("{} {}:{}", book_chirho, chapter_chirho, verse_num_chirho);

                        match loaded_module_chirho.read_entry_chirho(&ref_str_chirho) {
                            Ok(text_chirho) if !text_chirho.trim().is_empty() => {
                                let filtered_text_chirho = self.osis_filter_chirho
                                    .process_chirho(&text_chirho)
                                    .unwrap_or_else(|_| text_chirho.clone());
                                verses_chirho.push((
                                    verse_num_chirho.to_string(),
                                    filtered_text_chirho,
                                ));
                            }
                            _ => {
                                if verse_num_chirho > 1 {
                                    break;
                                }
                            }
                        }
                    }

                    if !verses_chirho.is_empty() {
                        return verses_chirho;
                    }
                }
            }

            // Fallback to sample verses
            get_sample_verses_chirho(book_chirho, chapter_chirho)
        }

        /// Set the current module
        pub fn set_module_chirho(&mut self, module_name_chirho: &str) {
            self.current_module_chirho = module_name_chirho.to_string();
            info!("Switched to module: {}", module_name_chirho);
        }

        /// Get current module name
        #[allow(dead_code)]
        pub fn current_module_chirho(&self) -> &str {
            &self.current_module_chirho
        }

        /// Get raw OSIS text for a verse (for interlinear parsing)
        pub fn get_raw_verse_chirho(&self, verse_ref_chirho: &str) -> Option<String> {
            if let Some(mgr_chirho) = &self.manager_chirho {
                if let Ok(loaded_module_chirho) = mgr_chirho.load_module_chirho(&self.current_module_chirho) {
                    if let Ok(text_chirho) = loaded_module_chirho.read_entry_chirho(verse_ref_chirho) {
                        if !text_chirho.trim().is_empty() {
                            return Some(text_chirho);
                        }
                    }
                }
            }
            None
        }

        /// Extract interlinear word data from OSIS text
        pub fn extract_interlinear_chirho(&self, verse_ref_chirho: &str) -> Vec<InterlinearWordChirho> {
            let mut words_chirho = Vec::new();

            // Try to get raw OSIS text
            if let Some(raw_text_chirho) = self.get_raw_verse_chirho(verse_ref_chirho) {
                // Parse <w> elements
                for cap_chirho in OSIS_WORD_REGEX_CHIRHO.captures_iter(&raw_text_chirho) {
                    let attrs_chirho = &cap_chirho[1];
                    let word_text_chirho = &cap_chirho[2];

                    // Extract gloss (translation)
                    let gloss_chirho = OSIS_GLOSS_REGEX_CHIRHO
                        .captures(attrs_chirho)
                        .map(|c_chirho| c_chirho[1].to_string())
                        .unwrap_or_default();

                    // Extract Strong's numbers
                    let strongs_chirho: Vec<String> = OSIS_STRONGS_REGEX_CHIRHO
                        .captures_iter(attrs_chirho)
                        .map(|c_chirho| c_chirho[1].to_string())
                        .collect();

                    // Extract morphology
                    let morph_chirho = OSIS_MORPH_REGEX_CHIRHO
                        .captures(attrs_chirho)
                        .map(|c_chirho| c_chirho[1].to_string())
                        .unwrap_or_default();

                    // Determine if Hebrew (H prefix) or Greek (G prefix)
                    let is_hebrew_chirho = strongs_chirho.first()
                        .map(|s_chirho: &String| s_chirho.starts_with('H'))
                        .unwrap_or(false);

                    // Extract part of speech from morphology
                    let pos_chirho = if !morph_chirho.is_empty() {
                        morph_chirho.split('|').next().unwrap_or("").trim().to_string()
                    } else {
                        String::new()
                    };

                    words_chirho.push(InterlinearWordChirho {
                        original_chirho: word_text_chirho.to_string().into(),
                        transliteration_chirho: "".into(), // Would need transliteration library
                        morphology_chirho: morph_chirho.into(),
                        strongs_chirho: strongs_chirho.join(", ").into(),
                        gloss_chirho: gloss_chirho.into(),
                        part_of_speech_chirho: pos_chirho.into(),
                        is_hebrew_chirho,
                    });
                }
            }

            words_chirho
        }

        /// Search for verses containing the query
        /// Searches only through sample verses for demo/fallback mode
        pub fn search_chirho(&self, query_chirho: &str, max_results_chirho: usize) -> Vec<(String, String)> {
            let query_lower_chirho = query_chirho.to_lowercase();
            let mut results_chirho = Vec::new();

            // Sample chapters to search (those with sample data)
            let sample_chapters_chirho: &[(&str, i32)] = &[
                ("Genesis", 1),
                ("John", 1),
                ("John", 3),
                ("Psalms", 23),
            ];

            // Search through sample chapters
            for (book_chirho, chapter_chirho) in sample_chapters_chirho {
                let verses_chirho = self.get_chapter_verses_chirho(book_chirho, *chapter_chirho);
                for (verse_num_chirho, text_chirho) in verses_chirho {
                    if text_chirho.to_lowercase().contains(&query_lower_chirho) {
                        let reference_chirho = format!("{} {}:{}", book_chirho, chapter_chirho, verse_num_chirho);
                        results_chirho.push((reference_chirho, text_chirho));

                        if results_chirho.len() >= max_results_chirho {
                            return results_chirho;
                        }
                    }
                }
            }

            results_chirho
        }
    }
}

// ============================================================================
// Sample Verses (for demo when no SWORD modules installed)
// ============================================================================

/// Convert raw verses to VerseChirho for parallel view (no highlights/notes)
fn raw_verses_to_verse_chirho(raw_verses_chirho: Vec<(String, String)>) -> Vec<VerseChirho> {
    raw_verses_chirho
        .into_iter()
        .map(|(ref_chirho, text_chirho)| VerseChirho {
            reference_chirho: ref_chirho.into(),
            text_chirho: text_chirho.into(),
            is_highlighted_chirho: false,
            highlight_color_chirho: "".into(),
            has_note_chirho: false,
            is_red_letter_chirho: false,
            section_heading_chirho: "".into(),
            poetry_indent_chirho: 0,
            is_paragraph_start_chirho: false,
            strongs_text_chirho: "".into(),
            has_strongs_chirho: false,
        })
        .collect()
}

/// Sample verses for demo (when no SWORD modules are installed)
fn get_sample_verses_chirho(book_chirho: &str, chapter_chirho: i32) -> Vec<(String, String)> {
    match (book_chirho, chapter_chirho) {
        ("Genesis", 1) => vec![
            ("1".into(), "In the beginning God created the heaven and the earth.".into()),
            ("2".into(), "And the earth was without form, and void; and darkness was upon the face of the deep. And the Spirit of God moved upon the face of the waters.".into()),
            ("3".into(), "And God said, Let there be light: and there was light.".into()),
            ("4".into(), "And God saw the light, that it was good: and God divided the light from the darkness.".into()),
            ("5".into(), "And God called the light Day, and the darkness he called Night. And the evening and the morning were the first day.".into()),
            ("6".into(), "And God said, Let there be a firmament in the midst of the waters, and let it divide the waters from the waters.".into()),
            ("7".into(), "And God made the firmament, and divided the waters which were under the firmament from the waters which were above the firmament: and it was so.".into()),
            ("8".into(), "And God called the firmament Heaven. And the evening and the morning were the second day.".into()),
            ("9".into(), "And God said, Let the waters under the heaven be gathered together unto one place, and let the dry land appear: and it was so.".into()),
            ("10".into(), "And God called the dry land Earth; and the gathering together of the waters called he Seas: and God saw that it was good.".into()),
        ],
        ("John", 1) => vec![
            ("1".into(), "Ἐν ἀρχῇ ἦν ὁ λόγος, καὶ ὁ λόγος ἦν πρὸς τὸν θεόν, καὶ θεὸς ἦν ὁ λόγος.".into()),
            ("2".into(), "οὗτος ἦν ἐν ἀρχῇ πρὸς τὸν θεόν.".into()),
            ("3".into(), "πάντα δι᾽ αὐτοῦ ἐγένετο, καὶ χωρὶς αὐτοῦ ἐγένετο οὐδὲ ἕν ὃ γέγονεν.".into()),
            ("4".into(), "ἐν αὐτῷ ζωὴ ἦν, καὶ ἡ ζωὴ ἦν τὸ φῶς τῶν ἀνθρώπων.".into()),
            ("5".into(), "καὶ τὸ φῶς ἐν τῇ σκοτίᾳ φαίνει, καὶ ἡ σκοτία αὐτὸ οὐ κατέλαβεν.".into()),
        ],
        ("John", 3) => vec![
            ("14".into(), "And as Moses lifted up the serpent in the wilderness, even so must the Son of man be lifted up:".into()),
            ("15".into(), "That whosoever believeth in him should not perish, but have eternal life.".into()),
            ("16".into(), "For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life.".into()),
            ("17".into(), "For God sent not his Son into the world to condemn the world; but that the world through him might be saved.".into()),
            ("18".into(), "He that believeth on him is not condemned: but he that believeth not is condemned already, because he hath not believed in the name of the only begotten Son of God.".into()),
        ],
        ("Psalms", 23) => vec![
            ("1".into(), "יְהוָ֥ה רֹ֝עִ֗י לֹ֣א אֶחְסָֽר׃ The LORD is my shepherd; I shall not want.".into()),
            ("2".into(), "בִּנְא֣וֹת דֶּ֭שֶׁא יַרְבִּיצֵ֑נִי עַל־מֵ֖י מְנֻח֣וֹת יְנַהֲלֵֽנִי׃ He maketh me to lie down in green pastures: he leadeth me beside the still waters.".into()),
            ("3".into(), "נַפְשִׁ֥י יְשׁוֹבֵ֑ב יַֽנְחֵ֥נִי בְמַעְגְּלֵי־צֶ֝֗דֶק לְמַ֣עַן שְׁמֽוֹ׃ He restoreth my soul: he leadeth me in the paths of righteousness for his name's sake.".into()),
            ("4".into(), "גַּ֤ם כִּֽי־אֵלֵ֨ךְ בְּגֵ֪יא צַלְמָ֡וֶת לֹא־אִ֘ירָ֤א רָ֗ע כִּי־אַתָּ֥ה עִמָּדִ֑י שִׁבְטְךָ֥ וּ֝מִשְׁעַנְתֶּ֗ךָ הֵ֣מָּה יְנַֽחֲמֻֽנִי׃ Yea, though I walk through the valley of the shadow of death, I will fear no evil: for thou art with me; thy rod and thy staff they comfort me.".into()),
            ("5".into(), "תַּעֲרֹ֬ךְ לְפָנַ֨י ׀ שֻׁלְחָ֗ן נֶ֥גֶד צֹרְרָ֑י דִּשַּׁ֖נְתָּ בַשֶּׁ֥מֶן רֹ֝אשִׁ֗י כּוֹסִ֥י רְוָיָֽה׃ Thou preparest a table before me in the presence of mine enemies: thou anointest my head with oil; my cup runneth over.".into()),
            ("6".into(), "אַ֤ךְ ׀ ט֤וֹב וָחֶ֣סֶד יִ֭רְדְּפוּנִי כָּל־יְמֵ֣י חַיָּ֑י וְשַׁבְתִּ֥י בְּבֵית־יְ֝הוָ֗ה לְאֹ֣רֶךְ יָמִֽים׃ Surely goodness and mercy shall follow me all the days of my life: and I will dwell in the house of the LORD for ever.".into()),
        ],
        _ => vec![
            ("1".into(), format!("Sample verse for {} chapter {}. Install SWORD modules for actual Bible text.", book_chirho, chapter_chirho)),
            ("2".into(), "Use installmgr_chirho to download Bible modules from CrossWire.".into()),
            ("3".into(), "Modules will appear in your ~/.sword directory.".into()),
        ],
    }
}

// ============================================================================
// Application Backend
// ============================================================================

/// Application state manager
struct AppBackendChirho {
    current_book_chirho: String,
    current_chapter_chirho: i32,
    current_module_chirho: String,
    db_conn_chirho: Connection,
    bible_engine_chirho: bible_engine_chirho::BibleEngineChirho,
}

impl AppBackendChirho {
    /// Create a new application backend with default state
    fn new_chirho() -> Result<Self> {
        // Get application data directory
        let data_dir_chirho = if let Some(proj_dirs_chirho) = ProjectDirs::from(
            APP_QUALIFIER_CHIRHO,
            APP_ORGANIZATION_CHIRHO,
            APP_NAME_CHIRHO,
        ) {
            proj_dirs_chirho.data_dir().to_path_buf()
        } else {
            PathBuf::from(".")
        };

        let db_path_chirho = data_dir_chirho.join(DATABASE_FILENAME_CHIRHO);
        let db_conn_chirho = database_chirho::init_database_chirho(&db_path_chirho)?;

        // Load saved state from settings
        let current_module_chirho = database_chirho::get_setting_chirho(&db_conn_chirho, "current_module")
            .unwrap_or_else(|| DEFAULT_MODULE_CHIRHO.to_string());
        let current_book_chirho = database_chirho::get_setting_chirho(&db_conn_chirho, "current_book")
            .unwrap_or_else(|| DEFAULT_BOOK_CHIRHO.to_string());
        let current_chapter_chirho: i32 = database_chirho::get_setting_chirho(&db_conn_chirho, "current_chapter")
            .and_then(|s_chirho| s_chirho.parse().ok())
            .unwrap_or(DEFAULT_CHAPTER_CHIRHO);

        let mut bible_engine_chirho = bible_engine_chirho::BibleEngineChirho::new_chirho();
        bible_engine_chirho.set_module_chirho(&current_module_chirho);

        Ok(Self {
            current_book_chirho,
            current_chapter_chirho,
            current_module_chirho,
            db_conn_chirho,
            bible_engine_chirho,
        })
    }

    /// Get verses for the current book and chapter
    fn get_verses_chirho(&self) -> Vec<VerseChirho> {
        let raw_verses_chirho = self.bible_engine_chirho.get_chapter_verses_chirho(
            &self.current_book_chirho,
            self.current_chapter_chirho,
        );

        // Get highlights with colors for this chapter
        let highlights_chirho = database_chirho::get_highlights_with_colors_chirho(
            &self.db_conn_chirho,
            &self.current_module_chirho,
            &self.current_book_chirho,
            self.current_chapter_chirho,
        ).unwrap_or_default();

        // Build a map of verse -> color
        let highlight_map_chirho: std::collections::HashMap<i32, String> = highlights_chirho
            .into_iter()
            .map(|h_chirho| (h_chirho.verse_chirho, h_chirho.color_chirho))
            .collect();

        let notes_chirho = database_chirho::get_verses_with_notes_chirho(
            &self.db_conn_chirho,
            &self.current_module_chirho,
            &self.current_book_chirho,
            self.current_chapter_chirho,
        ).unwrap_or_default();

        raw_verses_chirho
            .into_iter()
            .map(|(ref_chirho, text_chirho)| {
                let verse_num_chirho: i32 = ref_chirho.parse().unwrap_or(0);
                let highlight_color_chirho = highlight_map_chirho.get(&verse_num_chirho)
                    .cloned()
                    .unwrap_or_default();
                VerseChirho {
                    reference_chirho: ref_chirho.into(),
                    text_chirho: text_chirho.into(),
                    is_highlighted_chirho: highlight_map_chirho.contains_key(&verse_num_chirho),
                    highlight_color_chirho: highlight_color_chirho.into(),
                    has_note_chirho: notes_chirho.contains(&verse_num_chirho),
                    is_red_letter_chirho: false,
                    section_heading_chirho: "".into(),
                    poetry_indent_chirho: 0,
                    is_paragraph_start_chirho: false,
                    strongs_text_chirho: "".into(),
                    has_strongs_chirho: false,
                }
            })
            .collect()
    }

    /// Navigate to a new location
    fn navigate_to_chirho(&mut self, book_chirho: &str, chapter_chirho: i32) {
        self.current_book_chirho = book_chirho.to_string();
        self.current_chapter_chirho = chapter_chirho;

        // Save to settings
        let _ = database_chirho::set_setting_chirho(&self.db_conn_chirho, "current_book", book_chirho);
        let _ = database_chirho::set_setting_chirho(&self.db_conn_chirho, "current_chapter", &chapter_chirho.to_string());

        // Add to reading history
        let _ = database_chirho::add_reading_history_chirho(
            &self.db_conn_chirho,
            &self.current_module_chirho,
            book_chirho,
            chapter_chirho,
        );

        debug!("Navigated to {} {}", book_chirho, chapter_chirho);
    }

    /// Toggle highlight state for a verse
    fn toggle_highlight_chirho(&mut self, verse_ref_chirho: &str) -> bool {
        let verse_num_chirho: i32 = verse_ref_chirho.parse().unwrap_or(0);

        match database_chirho::toggle_highlight_chirho(
            &self.db_conn_chirho,
            &self.current_module_chirho,
            &self.current_book_chirho,
            self.current_chapter_chirho,
            verse_num_chirho,
        ) {
            Ok(is_highlighted_chirho) => {
                debug!("Toggled highlight for {} {}:{} = {}",
                    self.current_book_chirho, self.current_chapter_chirho, verse_num_chirho, is_highlighted_chirho);
                is_highlighted_chirho
            }
            Err(e_chirho) => {
                warn!("Failed to toggle highlight: {}", e_chirho);
                false
            }
        }
    }

    /// Set current module
    fn set_module_chirho(&mut self, module_name_chirho: &str) {
        self.current_module_chirho = module_name_chirho.to_string();
        self.bible_engine_chirho.set_module_chirho(module_name_chirho);
        let _ = database_chirho::set_setting_chirho(&self.db_conn_chirho, "current_module", module_name_chirho);
    }

    /// Get module names
    fn get_module_names_chirho(&self) -> Vec<String> {
        self.bible_engine_chirho.get_module_names_chirho()
    }

    /// Get the total number of chapters for a given book
    #[allow(dead_code)]
    fn get_chapter_count_chirho(book_chirho: &str) -> Option<i32> {
        BIBLE_BOOKS_CHIRHO
            .iter()
            .find(|(name_chirho, _)| *name_chirho == book_chirho)
            .map(|(_, count_chirho)| *count_chirho)
    }

    /// Check if a book name is valid
    #[allow(dead_code)]
    fn is_valid_book_chirho(book_chirho: &str) -> bool {
        BIBLE_BOOKS_CHIRHO.iter().any(|(name_chirho, _)| *name_chirho == book_chirho)
    }
}

// ============================================================================
// Main Application Entry Point
// ============================================================================

fn main() -> Result<(), slint::PlatformError> {
    // Initialize logging
    env_logger::init();
    info!("Starting Codex Lux Chirho...");

    // Create backend state
    let backend_chirho = match AppBackendChirho::new_chirho() {
        Ok(backend_chirho) => Rc::new(RefCell::new(backend_chirho)),
        Err(e_chirho) => {
            eprintln!("Failed to initialize backend: {}", e_chirho);
            return Err(slint::PlatformError::Other(e_chirho.to_string()));
        }
    };

    // Create the main window
    let main_window_chirho = MainWindowChirho::new()?;

    // Get the app state global
    let app_state_chirho = main_window_chirho.global::<AppStateChirho>();

    // Initialize books list with Old/New Testament grouping
    let books_model_chirho: Vec<BookChirho> = BIBLE_BOOKS_CHIRHO
        .iter()
        .enumerate()
        .map(|(index_chirho, (name_chirho, chapters_chirho))| BookChirho {
            name_chirho: (*name_chirho).into(),
            chapters_chirho: *chapters_chirho,
            is_old_testament_chirho: index_chirho < OLD_TESTAMENT_BOOK_COUNT_CHIRHO,
        })
        .collect();
    app_state_chirho.set_books_chirho(Rc::new(slint::VecModel::from(books_model_chirho)).into());

    // Initialize module names from actual available modules
    {
        let backend_ref_chirho = backend_chirho.borrow();
        let module_names_chirho: Vec<slint::SharedString> = backend_ref_chirho
            .get_module_names_chirho()
            .into_iter()
            .map(|s_chirho| s_chirho.into())
            .collect();
        app_state_chirho.set_module_names_chirho(Rc::new(slint::VecModel::from(module_names_chirho)).into());

        // Set current state from saved settings
        app_state_chirho.set_current_module_chirho(backend_ref_chirho.current_module_chirho.clone().into());
        app_state_chirho.set_current_book_chirho(backend_ref_chirho.current_book_chirho.clone().into());
        app_state_chirho.set_current_chapter_chirho(backend_ref_chirho.current_chapter_chirho);

        // Set chapter count for current book
        let chapter_count_chirho = BIBLE_BOOKS_CHIRHO
            .iter()
            .find(|(name_chirho, _)| *name_chirho == backend_ref_chirho.current_book_chirho.as_str())
            .map(|(_, count_chirho)| *count_chirho)
            .unwrap_or(50);
        app_state_chirho.set_current_book_chapter_count_chirho(chapter_count_chirho);
    }

    // Load initial verses
    {
        let verses_chirho = backend_chirho.borrow().get_verses_chirho();
        app_state_chirho.set_verses_chirho(Rc::new(slint::VecModel::from(verses_chirho)).into());
    }

    // Load initial bookmarks
    {
        let backend_ref_chirho = backend_chirho.borrow();
        let bookmarks_chirho = load_bookmarks_for_ui_chirho(&backend_ref_chirho.db_conn_chirho);
        app_state_chirho.set_bookmarks_chirho(Rc::new(slint::VecModel::from(bookmarks_chirho)).into());
    }

    // Set up navigation callback
    {
        let backend_clone_chirho = backend_chirho.clone();
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_navigate_to_chirho(move |book_chirho, chapter_chirho, _verse_chirho| {
            let mut backend_mut_chirho = backend_clone_chirho.borrow_mut();
            backend_mut_chirho.navigate_to_chirho(&book_chirho, chapter_chirho);

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                state_chirho.set_current_book_chirho(book_chirho.clone());
                state_chirho.set_current_chapter_chirho(chapter_chirho);

                // Update chapter count for current book
                let chapter_count_chirho = BIBLE_BOOKS_CHIRHO
                    .iter()
                    .find(|(name_chirho, _)| *name_chirho == book_chirho.as_str())
                    .map(|(_, count_chirho)| *count_chirho)
                    .unwrap_or(1);
                state_chirho.set_current_book_chapter_count_chirho(chapter_count_chirho);

                let verses_chirho = backend_mut_chirho.get_verses_chirho();
                state_chirho.set_verses_chirho(Rc::new(slint::VecModel::from(verses_chirho)).into());

                // Also update parallel verses if parallel view is enabled
                if state_chirho.get_parallel_view_enabled_chirho() {
                    let parallel_module_chirho = state_chirho.get_parallel_module_chirho();
                    if !parallel_module_chirho.is_empty() {
                        let raw_parallel_verses_chirho = backend_mut_chirho.bible_engine_chirho
                            .get_chapter_verses_for_module_chirho(
                                parallel_module_chirho.as_str(),
                                book_chirho.as_ref(),
                                chapter_chirho,
                            );
                        let parallel_verses_chirho = raw_verses_to_verse_chirho(raw_parallel_verses_chirho);
                        state_chirho.set_parallel_verses_chirho(
                            Rc::new(slint::VecModel::from(parallel_verses_chirho)).into()
                        );
                    }
                }

                state_chirho.set_status_message_chirho(
                    format!("Loaded {} {}", backend_mut_chirho.current_book_chirho, chapter_chirho).into()
                );
            }
        });
    }

    // Set up highlight toggle callback
    {
        let backend_clone_chirho = backend_chirho.clone();
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_toggle_highlight_chirho(move |reference_chirho| {
            let mut backend_mut_chirho = backend_clone_chirho.borrow_mut();
            backend_mut_chirho.toggle_highlight_chirho(&reference_chirho);

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                let verses_chirho = backend_mut_chirho.get_verses_chirho();
                state_chirho.set_verses_chirho(Rc::new(slint::VecModel::from(verses_chirho)).into());
            }
        });
    }

    // Set up highlight color callback
    {
        let backend_clone_chirho = backend_chirho.clone();
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_set_highlight_color_chirho(move |reference_chirho, color_chirho| {
            let backend_mut_chirho = backend_clone_chirho.borrow_mut();
            let verse_num_chirho: i32 = reference_chirho.parse().unwrap_or(1);

            // Add highlight with specific color
            let _ = database_chirho::add_highlight_with_color_chirho(
                &backend_mut_chirho.db_conn_chirho,
                &backend_mut_chirho.current_module_chirho,
                &backend_mut_chirho.current_book_chirho,
                backend_mut_chirho.current_chapter_chirho,
                verse_num_chirho,
                &color_chirho,
            );

            info!("Set highlight color {} for verse {}", color_chirho, verse_num_chirho);

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                let verses_chirho = backend_mut_chirho.get_verses_chirho();
                state_chirho.set_verses_chirho(Rc::new(slint::VecModel::from(verses_chirho)).into());
            }
        });
    }

    // Set up search callback
    {
        let backend_clone_chirho = backend_chirho.clone();
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_search_chirho(move |query_chirho| {
            info!("Search query: {}", query_chirho);

            if query_chirho.trim().is_empty() {
                if let Some(window_chirho) = window_weak_chirho.upgrade() {
                    let state_chirho = window_chirho.global::<AppStateChirho>();
                    state_chirho.set_search_results_chirho(Rc::new(slint::VecModel::from(Vec::<VerseChirho>::new())).into());
                    state_chirho.set_status_message_chirho("Enter a search term".into());
                }
                return;
            }

            let backend_ref_chirho = backend_clone_chirho.borrow();
            let search_results_chirho = backend_ref_chirho.bible_engine_chirho.search_chirho(&query_chirho, 50);

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();

                let results_chirho: Vec<VerseChirho> = search_results_chirho
                    .into_iter()
                    .map(|(ref_chirho, text_chirho)| VerseChirho {
                        reference_chirho: ref_chirho.into(),
                        text_chirho: text_chirho.into(),
                        is_highlighted_chirho: false,
                        highlight_color_chirho: slint::SharedString::default(),
                        has_note_chirho: false,
                        is_red_letter_chirho: false,
                        section_heading_chirho: slint::SharedString::default(),
                        poetry_indent_chirho: 0,
                        is_paragraph_start_chirho: false,
                        strongs_text_chirho: slint::SharedString::default(),
                        has_strongs_chirho: false,
                    })
                    .collect();

                let count_chirho = results_chirho.len();
                state_chirho.set_search_results_chirho(Rc::new(slint::VecModel::from(results_chirho)).into());
                state_chirho.set_status_message_chirho(
                    format!("Found {} result{} for '{}'", count_chirho, if count_chirho == 1 { "" } else { "s" }, query_chirho).into()
                );
            }
        });
    }

    // Set up module loading callback
    {
        let backend_clone_chirho = backend_chirho.clone();
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_load_module_chirho(move |module_chirho| {
            info!("Loading module: {}", module_chirho);

            let mut backend_mut_chirho = backend_clone_chirho.borrow_mut();
            backend_mut_chirho.set_module_chirho(&module_chirho);

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                state_chirho.set_current_module_chirho(module_chirho.clone());

                // Reload verses with new module
                let verses_chirho = backend_mut_chirho.get_verses_chirho();
                state_chirho.set_verses_chirho(Rc::new(slint::VecModel::from(verses_chirho)).into());

                state_chirho.set_status_message_chirho(format!("Loaded module: {}", module_chirho).into());
            }
        });
    }

    // Set up goto verse callback
    {
        let backend_clone_chirho = backend_chirho.clone();
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_goto_verse_chirho(move |reference_chirho| {
            info!("Goto verse: {}", reference_chirho);

            // Parse reference like "John 3:16" or "Gen 1:1"
            if let Some((book_chirho, chapter_chirho, _verse_chirho)) = parse_reference_chirho(&reference_chirho) {
                let mut backend_mut_chirho = backend_clone_chirho.borrow_mut();
                backend_mut_chirho.navigate_to_chirho(&book_chirho, chapter_chirho);

                if let Some(window_chirho) = window_weak_chirho.upgrade() {
                    let state_chirho = window_chirho.global::<AppStateChirho>();
                    state_chirho.set_current_book_chirho(book_chirho.clone().into());
                    state_chirho.set_current_chapter_chirho(chapter_chirho);

                    let verses_chirho = backend_mut_chirho.get_verses_chirho();
                    state_chirho.set_verses_chirho(Rc::new(slint::VecModel::from(verses_chirho)).into());

                    state_chirho.set_status_message_chirho(
                        format!("Navigated to {} {}", book_chirho, chapter_chirho).into()
                    );
                }
            } else if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                state_chirho.set_status_message_chirho(
                    format!("Could not parse reference: {}", reference_chirho).into()
                );
            }
        });
    }

    // Set up theme toggle callback
    {
        let backend_clone_chirho = backend_chirho.clone();

        app_state_chirho.on_toggle_theme_chirho(move || {
            let backend_ref_chirho = backend_clone_chirho.borrow();
            // Save theme preference to settings
            let current_theme_chirho = database_chirho::get_setting_chirho(&backend_ref_chirho.db_conn_chirho, "theme")
                .unwrap_or_else(|| "dark".to_string());
            let new_theme_chirho = if current_theme_chirho == "dark" { "light" } else { "dark" };
            let _ = database_chirho::set_setting_chirho(&backend_ref_chirho.db_conn_chirho, "theme", new_theme_chirho);
            info!("Theme changed to: {}", new_theme_chirho);
        });
    }

    // Set up bookmark toggle callback
    {
        let backend_clone_chirho = backend_chirho.clone();
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_toggle_bookmark_chirho(move |verse_ref_chirho| {
            let backend_ref_chirho = backend_clone_chirho.borrow();
            let verse_num_chirho: i32 = verse_ref_chirho.parse().unwrap_or(1);

            // Check if already bookmarked
            if let Some(id_chirho) = database_chirho::get_bookmark_id_chirho(
                &backend_ref_chirho.db_conn_chirho,
                &backend_ref_chirho.current_module_chirho,
                &backend_ref_chirho.current_book_chirho,
                backend_ref_chirho.current_chapter_chirho,
                verse_num_chirho,
            ) {
                // Remove bookmark
                let _ = database_chirho::remove_bookmark_chirho(&backend_ref_chirho.db_conn_chirho, id_chirho);
                info!("Removed bookmark at {}:{}:{}", backend_ref_chirho.current_book_chirho, backend_ref_chirho.current_chapter_chirho, verse_num_chirho);
            } else {
                // Add bookmark
                let reference_label_chirho = format!(
                    "{} {}:{}",
                    backend_ref_chirho.current_book_chirho,
                    backend_ref_chirho.current_chapter_chirho,
                    verse_num_chirho
                );
                let _ = database_chirho::add_bookmark_chirho(
                    &backend_ref_chirho.db_conn_chirho,
                    &backend_ref_chirho.current_module_chirho,
                    &backend_ref_chirho.current_book_chirho,
                    backend_ref_chirho.current_chapter_chirho,
                    verse_num_chirho,
                    Some(&reference_label_chirho),
                );
                info!("Added bookmark at {}:{}:{}", backend_ref_chirho.current_book_chirho, backend_ref_chirho.current_chapter_chirho, verse_num_chirho);
            }

            // Refresh bookmarks list
            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                let bookmarks_chirho = load_bookmarks_for_ui_chirho(&backend_ref_chirho.db_conn_chirho);
                state_chirho.set_bookmarks_chirho(Rc::new(slint::VecModel::from(bookmarks_chirho)).into());
                state_chirho.set_status_message_chirho("Bookmark updated".into());
            }
        });
    }

    // Set up update bookmark label callback
    {
        let backend_clone_chirho = backend_chirho.clone();
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_update_bookmark_label_chirho(move |id_chirho, label_chirho| {
            info!("Updating bookmark label: {} -> {}", id_chirho, label_chirho);
            let backend_ref_chirho = backend_clone_chirho.borrow();

            match database_chirho::update_bookmark_label_chirho(&backend_ref_chirho.db_conn_chirho, id_chirho as i64, label_chirho.as_str()) {
                Ok(_) => {
                    if let Some(window_chirho) = window_weak_chirho.upgrade() {
                        let state_chirho = window_chirho.global::<AppStateChirho>();
                        let bookmarks_chirho = load_bookmarks_for_ui_chirho(&backend_ref_chirho.db_conn_chirho);
                        state_chirho.set_bookmarks_chirho(Rc::new(slint::VecModel::from(bookmarks_chirho)).into());
                        state_chirho.set_status_message_chirho("Bookmark label updated".into());
                    }
                }
                Err(e_chirho) => {
                    warn!("Failed to update bookmark label: {}", e_chirho);
                }
            }
        });
    }

    // Set up copy verse callback
    {
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_copy_verse_chirho(move |reference_chirho, text_chirho| {
            // Get copy format settings from app state
            let format_index_chirho = if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                state_chirho.get_copy_format_index_chirho()
            } else {
                0
            };

            // Format text based on settings:
            // 0 = "text - reference", 1 = "reference: text", 2 = "text only"
            let formatted_text_chirho = match format_index_chirho {
                1 => format!("{}: {}", reference_chirho, text_chirho),
                2 => text_chirho.to_string(),
                _ => format!("{} - {}", text_chirho, reference_chirho), // default
            };

            if copy_to_clipboard_chirho(&formatted_text_chirho) {
                info!("Copied verse to clipboard: {}", reference_chirho);
                if let Some(window_chirho) = window_weak_chirho.upgrade() {
                    let state_chirho = window_chirho.global::<AppStateChirho>();
                    state_chirho.set_status_message_chirho(
                        format!("Copied {} to clipboard", reference_chirho).into()
                    );
                }
            } else {
                warn!("Failed to copy to clipboard");
            }
        });
    }

    // Set up save note callback
    {
        let backend_clone_chirho = backend_chirho.clone();
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_save_note_chirho(move |verse_ref_chirho, content_chirho| {
            let backend_ref_chirho = backend_clone_chirho.borrow();
            let verse_num_chirho: i32 = verse_ref_chirho.parse().unwrap_or(1);

            if content_chirho.trim().is_empty() {
                // Delete note if content is empty
                let _ = database_chirho::delete_note_chirho(
                    &backend_ref_chirho.db_conn_chirho,
                    &backend_ref_chirho.current_module_chirho,
                    &backend_ref_chirho.current_book_chirho,
                    backend_ref_chirho.current_chapter_chirho,
                    verse_num_chirho,
                );
                info!("Deleted note at {}:{}:{}", backend_ref_chirho.current_book_chirho, backend_ref_chirho.current_chapter_chirho, verse_num_chirho);
            } else {
                // Save note
                let _ = database_chirho::save_note_chirho(
                    &backend_ref_chirho.db_conn_chirho,
                    &backend_ref_chirho.current_module_chirho,
                    &backend_ref_chirho.current_book_chirho,
                    backend_ref_chirho.current_chapter_chirho,
                    verse_num_chirho,
                    &content_chirho,
                );
                info!("Saved note at {}:{}:{}", backend_ref_chirho.current_book_chirho, backend_ref_chirho.current_chapter_chirho, verse_num_chirho);
            }

            // Refresh verses to update note indicator
            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                // Can't call get_verses_chirho here because we have immutable borrow
                // Will refresh on next navigation
                state_chirho.set_status_message_chirho(
                    if content_chirho.trim().is_empty() { "Note deleted".into() } else { "Note saved".into() }
                );
            }
        });
    }

    // Set up get note callback
    {
        let backend_clone_chirho = backend_chirho.clone();
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_get_note_chirho(move |verse_ref_chirho| {
            let backend_ref_chirho = backend_clone_chirho.borrow();
            let verse_num_chirho: i32 = verse_ref_chirho.parse().unwrap_or(1);

            let note_content_chirho = database_chirho::get_note_chirho(
                &backend_ref_chirho.db_conn_chirho,
                &backend_ref_chirho.current_module_chirho,
                &backend_ref_chirho.current_book_chirho,
                backend_ref_chirho.current_chapter_chirho,
                verse_num_chirho,
            ).map(|n_chirho| n_chirho.content_chirho).unwrap_or_default();

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                state_chirho.set_note_content_chirho(note_content_chirho.into());
            }
        });
    }

    // Set up history callback
    {
        let backend_clone_chirho = backend_chirho.clone();
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_load_history_chirho(move || {
            let backend_ref_chirho = backend_clone_chirho.borrow();
            let history_entries_chirho = database_chirho::get_recent_history_chirho(&backend_ref_chirho.db_conn_chirho, 20)
                .unwrap_or_default();

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                let ui_entries_chirho: Vec<HistoryEntryChirho> = history_entries_chirho
                    .into_iter()
                    .map(|e_chirho| HistoryEntryChirho {
                        book_chirho: e_chirho.book_chirho.into(),
                        chapter_chirho: e_chirho.chapter_chirho,
                        timestamp_chirho: format_timestamp_chirho(&e_chirho.timestamp_chirho).into(),
                    })
                    .collect();
                state_chirho.set_history_entries_chirho(Rc::new(slint::VecModel::from(ui_entries_chirho)).into());
            }
        });
    }

    // Set up navigate to reference callback (for search results)
    {
        let backend_clone_chirho = backend_chirho.clone();
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_navigate_to_reference_chirho(move |reference_chirho| {
            if let Some((book_chirho, chapter_chirho, _verse_chirho)) = parse_reference_chirho(&reference_chirho) {
                let mut backend_mut_chirho = backend_clone_chirho.borrow_mut();
                backend_mut_chirho.navigate_to_chirho(&book_chirho, chapter_chirho);

                if let Some(window_chirho) = window_weak_chirho.upgrade() {
                    let state_chirho = window_chirho.global::<AppStateChirho>();
                    state_chirho.set_current_book_chirho(book_chirho.clone().into());
                    state_chirho.set_current_chapter_chirho(chapter_chirho);

                    // Update chapter count
                    let chapter_count_chirho = BIBLE_BOOKS_CHIRHO
                        .iter()
                        .find(|(name_chirho, _)| *name_chirho == book_chirho.as_str())
                        .map(|(_, count_chirho)| *count_chirho)
                        .unwrap_or(1);
                    state_chirho.set_current_book_chapter_count_chirho(chapter_count_chirho);

                    let verses_chirho = backend_mut_chirho.get_verses_chirho();
                    state_chirho.set_verses_chirho(Rc::new(slint::VecModel::from(verses_chirho)).into());

                    state_chirho.set_status_message_chirho(
                        format!("Navigated to {}", reference_chirho).into()
                    );
                }
            } else if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                state_chirho.set_status_message_chirho(
                    format!("Could not parse: {}", reference_chirho).into()
                );
            }
        });
    }

    // Set up navigate to previous book callback
    {
        let backend_clone_chirho = backend_chirho.clone();
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_navigate_prev_book_chirho(move || {
            let mut backend_mut_chirho = backend_clone_chirho.borrow_mut();
            let current_book_chirho = backend_mut_chirho.current_book_chirho.clone();

            // Find current book index
            if let Some(current_index_chirho) = BIBLE_BOOKS_CHIRHO
                .iter()
                .position(|(name_chirho, _)| *name_chirho == current_book_chirho)
            {
                // Get previous book (wrap to Revelation if at Genesis)
                let prev_index_chirho = if current_index_chirho == 0 {
                    BIBLE_BOOKS_CHIRHO.len() - 1
                } else {
                    current_index_chirho - 1
                };

                let (prev_book_chirho, _) = BIBLE_BOOKS_CHIRHO[prev_index_chirho];
                backend_mut_chirho.navigate_to_chirho(prev_book_chirho, 1);

                if let Some(window_chirho) = window_weak_chirho.upgrade() {
                    let state_chirho = window_chirho.global::<AppStateChirho>();
                    state_chirho.set_current_book_chirho(prev_book_chirho.into());
                    state_chirho.set_current_chapter_chirho(1);

                    // Update chapter count
                    let chapter_count_chirho = BIBLE_BOOKS_CHIRHO
                        .iter()
                        .find(|(name_chirho, _)| *name_chirho == prev_book_chirho)
                        .map(|(_, count_chirho)| *count_chirho)
                        .unwrap_or(1);
                    state_chirho.set_current_book_chapter_count_chirho(chapter_count_chirho);

                    let verses_chirho = backend_mut_chirho.get_verses_chirho();
                    state_chirho.set_verses_chirho(Rc::new(slint::VecModel::from(verses_chirho)).into());

                    state_chirho.set_status_message_chirho(
                        format!("Navigated to {} 1", prev_book_chirho).into()
                    );
                }
            }
        });
    }

    // Set up navigate to next book callback
    {
        let backend_clone_chirho = backend_chirho.clone();
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_navigate_next_book_chirho(move || {
            let mut backend_mut_chirho = backend_clone_chirho.borrow_mut();
            let current_book_chirho = backend_mut_chirho.current_book_chirho.clone();

            // Find current book index
            if let Some(current_index_chirho) = BIBLE_BOOKS_CHIRHO
                .iter()
                .position(|(name_chirho, _)| *name_chirho == current_book_chirho)
            {
                // Get next book (wrap to Genesis if at Revelation)
                let next_index_chirho = if current_index_chirho >= BIBLE_BOOKS_CHIRHO.len() - 1 {
                    0
                } else {
                    current_index_chirho + 1
                };

                let (next_book_chirho, _) = BIBLE_BOOKS_CHIRHO[next_index_chirho];
                backend_mut_chirho.navigate_to_chirho(next_book_chirho, 1);

                if let Some(window_chirho) = window_weak_chirho.upgrade() {
                    let state_chirho = window_chirho.global::<AppStateChirho>();
                    state_chirho.set_current_book_chirho(next_book_chirho.into());
                    state_chirho.set_current_chapter_chirho(1);

                    // Update chapter count
                    let chapter_count_chirho = BIBLE_BOOKS_CHIRHO
                        .iter()
                        .find(|(name_chirho, _)| *name_chirho == next_book_chirho)
                        .map(|(_, count_chirho)| *count_chirho)
                        .unwrap_or(1);
                    state_chirho.set_current_book_chapter_count_chirho(chapter_count_chirho);

                    let verses_chirho = backend_mut_chirho.get_verses_chirho();
                    state_chirho.set_verses_chirho(Rc::new(slint::VecModel::from(verses_chirho)).into());

                    state_chirho.set_status_message_chirho(
                        format!("Navigated to {} 1", next_book_chirho).into()
                    );
                }
            }
        });
    }

    // Set up load all highlights callback
    {
        let backend_clone_chirho = backend_chirho.clone();
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_load_all_highlights_chirho(move || {
            let backend_ref_chirho = backend_clone_chirho.borrow();
            let highlights_chirho = load_highlights_for_ui_chirho(&backend_ref_chirho.db_conn_chirho);

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                state_chirho.set_all_highlights_chirho(Rc::new(slint::VecModel::from(highlights_chirho)).into());
            }
        });
    }

    // Set up load all notes callback
    {
        let backend_clone_chirho = backend_chirho.clone();
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_load_all_notes_chirho(move || {
            let backend_ref_chirho = backend_clone_chirho.borrow();
            let notes_chirho = load_notes_for_ui_chirho(&backend_ref_chirho.db_conn_chirho);

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                state_chirho.set_all_notes_chirho(Rc::new(slint::VecModel::from(notes_chirho)).into());
            }
        });
    }

    // Set up delete note callback (from notes panel)
    {
        let backend_clone_chirho = backend_chirho.clone();
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_delete_note_chirho(move |reference_chirho| {
            if let Some((book_chirho, chapter_chirho, verse_chirho)) = parse_reference_chirho(&reference_chirho) {
                let backend_ref_chirho = backend_clone_chirho.borrow();
                if let Err(e_chirho) = database_chirho::delete_note_chirho(
                    &backend_ref_chirho.db_conn_chirho,
                    &backend_ref_chirho.current_module_chirho,
                    &book_chirho,
                    chapter_chirho,
                    verse_chirho,
                ) {
                    warn!("Failed to delete note: {}", e_chirho);
                }

                // Refresh notes list
                let notes_chirho = load_notes_for_ui_chirho(&backend_ref_chirho.db_conn_chirho);
                if let Some(window_chirho) = window_weak_chirho.upgrade() {
                    let state_chirho = window_chirho.global::<AppStateChirho>();
                    state_chirho.set_all_notes_chirho(Rc::new(slint::VecModel::from(notes_chirho)).into());
                    state_chirho.set_status_message_chirho(
                        format!("Deleted note for {}", reference_chirho).into()
                    );
                }
            }
        });
    }

    // Set up refresh remote modules callback
    {
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_refresh_remote_modules_chirho(move || {
            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                state_chirho.set_module_manager_loading_chirho(true);
                state_chirho.set_module_manager_status_chirho("Connecting to CrossWire...".into());

                // For now, show a sample list of popular modules
                // Full integration with rsword_chirho's install_mgr would require async
                let sample_modules_chirho: Vec<RemoteModuleChirho> = vec![
                    RemoteModuleChirho {
                        name_chirho: "KJV".into(),
                        description_chirho: "King James Version".into(),
                        language_chirho: "English".into(),
                        type_chirho: "Bible".into(),
                        is_installed_chirho: true,
                    },
                    RemoteModuleChirho {
                        name_chirho: "ESV".into(),
                        description_chirho: "English Standard Version".into(),
                        language_chirho: "English".into(),
                        type_chirho: "Bible".into(),
                        is_installed_chirho: false,
                    },
                    RemoteModuleChirho {
                        name_chirho: "SBLGNT".into(),
                        description_chirho: "SBL Greek New Testament".into(),
                        language_chirho: "Greek".into(),
                        type_chirho: "Bible".into(),
                        is_installed_chirho: false,
                    },
                    RemoteModuleChirho {
                        name_chirho: "WLC".into(),
                        description_chirho: "Westminster Leningrad Codex".into(),
                        language_chirho: "Hebrew".into(),
                        type_chirho: "Bible".into(),
                        is_installed_chirho: false,
                    },
                    RemoteModuleChirho {
                        name_chirho: "StrongsGreek".into(),
                        description_chirho: "Strong's Greek Dictionary".into(),
                        language_chirho: "Greek".into(),
                        type_chirho: "Lexicon".into(),
                        is_installed_chirho: false,
                    },
                    RemoteModuleChirho {
                        name_chirho: "StrongsHebrew".into(),
                        description_chirho: "Strong's Hebrew Dictionary".into(),
                        language_chirho: "Hebrew".into(),
                        type_chirho: "Lexicon".into(),
                        is_installed_chirho: false,
                    },
                    RemoteModuleChirho {
                        name_chirho: "MHCC".into(),
                        description_chirho: "Matthew Henry Concise Commentary".into(),
                        language_chirho: "English".into(),
                        type_chirho: "Commentary".into(),
                        is_installed_chirho: false,
                    },
                ];

                state_chirho.set_remote_modules_chirho(Rc::new(slint::VecModel::from(sample_modules_chirho)).into());
                state_chirho.set_module_manager_loading_chirho(false);
                state_chirho.set_module_manager_status_chirho("Ready".into());
            }
        });
    }

    // Set up install module callback
    {
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_install_module_chirho(move |module_name_chirho| {
            info!("Request to install module: {}", module_name_chirho);

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                state_chirho.set_module_manager_status_chirho(
                    format!("Installing {}...", module_name_chirho).into()
                );

                // TODO: Integrate with rsword_chirho's InstallMgrChirho for actual installation
                // For now, just update the UI
                state_chirho.set_status_message_chirho(
                    "Module installation requires rsword_chirho InstallMgr (coming soon)".to_string().into()
                );
            }
        });
    }

    // Set up uninstall module callback
    {
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_uninstall_module_chirho(move |module_name_chirho| {
            info!("Request to uninstall module: {}", module_name_chirho);

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                state_chirho.set_module_manager_status_chirho(
                    format!("Uninstalling {}...", module_name_chirho).into()
                );

                // TODO: Integrate with rsword_chirho's InstallMgrChirho for actual uninstallation
                state_chirho.set_status_message_chirho(
                    "Module uninstallation requires rsword_chirho InstallMgr (coming soon)".to_string().into()
                );
            }
        });
    }

    // Set up quick navigation search callback
    {
        let backend_clone_chirho = backend_chirho.clone();
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_quick_nav_search_chirho(move |query_chirho| {
            let backend_ref_chirho = backend_clone_chirho.borrow();
            let results_chirho = quick_nav_search_chirho(&query_chirho, &backend_ref_chirho.db_conn_chirho);

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                state_chirho.set_quick_nav_results_chirho(
                    Rc::new(slint::VecModel::from(results_chirho)).into()
                );
            }
        });
    }

    // Set up copy selected verses callback (for multi-verse selection)
    {
        let backend_clone_chirho = backend_chirho.clone();
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_copy_selected_verses_chirho(move || {
            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                let start_verse_chirho = state_chirho.get_selection_start_verse_chirho();
                let end_verse_chirho = state_chirho.get_selection_end_verse_chirho();
                let format_index_chirho = state_chirho.get_copy_format_index_chirho();
                let include_numbers_chirho = state_chirho.get_copy_include_verse_numbers_chirho();

                if start_verse_chirho > 0 && end_verse_chirho > 0 {
                    let backend_ref_chirho = backend_clone_chirho.borrow();
                    let verses_chirho = backend_ref_chirho.get_verses_chirho();

                    let min_verse_chirho = start_verse_chirho.min(end_verse_chirho) as usize;
                    let max_verse_chirho = start_verse_chirho.max(end_verse_chirho) as usize;

                    let mut text_parts_chirho: Vec<String> = Vec::new();

                    for verse_chirho in verses_chirho.iter() {
                        if let Ok(verse_num_chirho) = verse_chirho.reference_chirho.to_string().parse::<usize>() {
                            if verse_num_chirho >= min_verse_chirho && verse_num_chirho <= max_verse_chirho {
                                if include_numbers_chirho {
                                    text_parts_chirho.push(format!("{}. {}", verse_num_chirho, verse_chirho.text_chirho));
                                } else {
                                    text_parts_chirho.push(verse_chirho.text_chirho.to_string());
                                }
                            }
                        }
                    }

                    let reference_chirho = format!("{} {}:{}-{}",
                        backend_ref_chirho.current_book_chirho,
                        backend_ref_chirho.current_chapter_chirho,
                        min_verse_chirho,
                        max_verse_chirho
                    );

                    let verses_text_chirho = text_parts_chirho.join(" ");

                    let formatted_text_chirho = match format_index_chirho {
                        1 => format!("{}: {}", reference_chirho, verses_text_chirho),
                        2 => verses_text_chirho,
                        _ => format!("{} - {}", verses_text_chirho, reference_chirho),
                    };

                    if copy_to_clipboard_chirho(&formatted_text_chirho) {
                        state_chirho.set_status_message_chirho(
                            format!("Copied {} to clipboard", reference_chirho).into()
                        );
                    }

                    // Clear selection
                    state_chirho.set_selection_start_verse_chirho(0);
                    state_chirho.set_selection_end_verse_chirho(0);
                    state_chirho.set_verse_selection_mode_chirho(false);
                }
            }
        });
    }

    // Set up clear selection callback
    {
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_clear_selection_chirho(move || {
            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                state_chirho.set_selection_start_verse_chirho(0);
                state_chirho.set_selection_end_verse_chirho(0);
                state_chirho.set_verse_selection_mode_chirho(false);
            }
        });
    }

    // Set up onboarding complete callback
    {
        let backend_clone_chirho = backend_chirho.clone();
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_onboarding_complete_chirho(move || {
            let backend_ref_chirho = backend_clone_chirho.borrow();
            let _ = database_chirho::set_setting_chirho(
                &backend_ref_chirho.db_conn_chirho,
                "onboarding_complete",
                "true",
            );
            info!("Onboarding complete");

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                state_chirho.set_onboarding_visible_chirho(false);
                state_chirho.set_onboarding_step_chirho(0);
                state_chirho.set_status_message_chirho("Welcome to Codex Lux Chirho!".into());
            }
        });
    }

    // Set up onboarding skip callback
    {
        let backend_clone_chirho = backend_chirho.clone();
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_onboarding_skip_chirho(move || {
            let backend_ref_chirho = backend_clone_chirho.borrow();
            let _ = database_chirho::set_setting_chirho(
                &backend_ref_chirho.db_conn_chirho,
                "onboarding_complete",
                "true",
            );
            info!("Onboarding skipped");

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                state_chirho.set_onboarding_visible_chirho(false);
                state_chirho.set_onboarding_step_chirho(0);
            }
        });
    }

    // Set up export callbacks
    {
        let backend_clone_chirho = backend_chirho.clone();
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_export_highlights_chirho(move || {
            let backend_ref_chirho = backend_clone_chirho.borrow();
            match export_highlights_to_json_chirho(&backend_ref_chirho.db_conn_chirho) {
                Ok(path_chirho) => {
                    info!("Exported highlights to: {:?}", path_chirho);
                    if let Some(window_chirho) = window_weak_chirho.upgrade() {
                        let state_chirho = window_chirho.global::<AppStateChirho>();
                        state_chirho.set_status_message_chirho(
                            format!("Highlights exported to {:?}", path_chirho).into()
                        );
                    }
                }
                Err(e_chirho) => {
                    warn!("Failed to export highlights: {}", e_chirho);
                    if let Some(window_chirho) = window_weak_chirho.upgrade() {
                        let state_chirho = window_chirho.global::<AppStateChirho>();
                        state_chirho.set_status_message_chirho("Export failed".into());
                    }
                }
            }
        });
    }

    {
        let backend_clone_chirho = backend_chirho.clone();
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_export_bookmarks_chirho(move || {
            let backend_ref_chirho = backend_clone_chirho.borrow();
            match export_bookmarks_to_json_chirho(&backend_ref_chirho.db_conn_chirho) {
                Ok(path_chirho) => {
                    info!("Exported bookmarks to: {:?}", path_chirho);
                    if let Some(window_chirho) = window_weak_chirho.upgrade() {
                        let state_chirho = window_chirho.global::<AppStateChirho>();
                        state_chirho.set_status_message_chirho(
                            format!("Bookmarks exported to {:?}", path_chirho).into()
                        );
                    }
                }
                Err(e_chirho) => {
                    warn!("Failed to export bookmarks: {}", e_chirho);
                    if let Some(window_chirho) = window_weak_chirho.upgrade() {
                        let state_chirho = window_chirho.global::<AppStateChirho>();
                        state_chirho.set_status_message_chirho("Export failed".into());
                    }
                }
            }
        });
    }

    {
        let backend_clone_chirho = backend_chirho.clone();
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_export_notes_chirho(move || {
            let backend_ref_chirho = backend_clone_chirho.borrow();
            match export_notes_to_json_chirho(&backend_ref_chirho.db_conn_chirho) {
                Ok(path_chirho) => {
                    info!("Exported notes to: {:?}", path_chirho);
                    if let Some(window_chirho) = window_weak_chirho.upgrade() {
                        let state_chirho = window_chirho.global::<AppStateChirho>();
                        state_chirho.set_status_message_chirho(
                            format!("Notes exported to {:?}", path_chirho).into()
                        );
                    }
                }
                Err(e_chirho) => {
                    warn!("Failed to export notes: {}", e_chirho);
                    if let Some(window_chirho) = window_weak_chirho.upgrade() {
                        let state_chirho = window_chirho.global::<AppStateChirho>();
                        state_chirho.set_status_message_chirho("Export failed".into());
                    }
                }
            }
        });
    }

    {
        let backend_clone_chirho = backend_chirho.clone();
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_export_all_data_chirho(move || {
            let backend_ref_chirho = backend_clone_chirho.borrow();
            match export_all_data_to_json_chirho(&backend_ref_chirho.db_conn_chirho) {
                Ok(path_chirho) => {
                    info!("Exported all data to: {:?}", path_chirho);
                    if let Some(window_chirho) = window_weak_chirho.upgrade() {
                        let state_chirho = window_chirho.global::<AppStateChirho>();
                        state_chirho.set_status_message_chirho(
                            format!("All data exported to {:?}", path_chirho).into()
                        );
                    }
                }
                Err(e_chirho) => {
                    warn!("Failed to export data: {}", e_chirho);
                    if let Some(window_chirho) = window_weak_chirho.upgrade() {
                        let state_chirho = window_chirho.global::<AppStateChirho>();
                        state_chirho.set_status_message_chirho("Export failed".into());
                    }
                }
            }
        });
    }

    // Import data callback
    {
        let backend_clone_chirho = backend_chirho.clone();
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_import_data_chirho(move |filename_chirho| {
            info!("Importing data from: {}", filename_chirho);

            let backend_ref_chirho = backend_clone_chirho.borrow();

            // Get full path to the backup file
            if let Ok(export_dir_chirho) = get_export_directory_chirho() {
                let file_path_chirho = export_dir_chirho.join(filename_chirho.as_str());

                match import_data_from_json_chirho(&backend_ref_chirho.db_conn_chirho, file_path_chirho.to_str().unwrap_or("")) {
                    Ok((highlights_chirho, bookmarks_chirho, notes_chirho)) => {
                        if let Some(window_chirho) = window_weak_chirho.upgrade() {
                            let state_chirho = window_chirho.global::<AppStateChirho>();
                            state_chirho.set_status_message_chirho(
                                format!("Imported: {} highlights, {} bookmarks, {} notes",
                                    highlights_chirho, bookmarks_chirho, notes_chirho).into()
                            );
                        }
                    }
                    Err(e_chirho) => {
                        warn!("Failed to import data: {}", e_chirho);
                        if let Some(window_chirho) = window_weak_chirho.upgrade() {
                            let state_chirho = window_chirho.global::<AppStateChirho>();
                            state_chirho.set_status_message_chirho("Import failed".into());
                        }
                    }
                }
            }
        });
    }

    // List backup files callback
    {
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_load_backup_files_chirho(move || {
            info!("Loading backup files list");

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();

                match list_backup_files_chirho() {
                    Ok(files_chirho) => {
                        let backup_list_chirho: Vec<slint::SharedString> = files_chirho
                            .iter()
                            .map(|f_chirho| f_chirho.clone().into())
                            .collect();

                        state_chirho.set_backup_files_chirho(
                            Rc::new(slint::VecModel::from(backup_list_chirho)).into()
                        );
                    }
                    Err(e_chirho) => {
                        warn!("Failed to list backup files: {}", e_chirho);
                    }
                }
            }
        });
    }

    // Set up parallel module loading callback
    {
        let backend_clone_chirho = backend_chirho.clone();
        let window_weak_chirho = main_window_chirho.as_weak();
        app_state_chirho.on_load_parallel_module_chirho(move |module_name_chirho| {
            info!("Loading parallel module: {}", module_name_chirho);

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                let backend_ref_chirho = backend_clone_chirho.borrow();

                // Get the current book and chapter
                let current_book_chirho = state_chirho.get_current_book_chirho();
                let current_chapter_chirho = state_chirho.get_current_chapter_chirho();

                // Load verses for the parallel module using BibleEngine
                let raw_parallel_verses_chirho = backend_ref_chirho.bible_engine_chirho
                    .get_chapter_verses_for_module_chirho(
                        module_name_chirho.as_str(),
                        current_book_chirho.as_ref(),
                        current_chapter_chirho,
                    );
                let parallel_verses_chirho = raw_verses_to_verse_chirho(raw_parallel_verses_chirho);

                state_chirho.set_parallel_module_chirho(module_name_chirho.clone());
                state_chirho.set_parallel_verses_chirho(Rc::new(slint::VecModel::from(parallel_verses_chirho)).into());
                state_chirho.set_status_message_chirho(
                    format!("Parallel view: {} vs {}",
                        state_chirho.get_current_module_chirho(),
                        module_name_chirho
                    ).into()
                );
            }
        });
    }

    // Set up load verse info callback (for info panel)
    {
        let backend_clone_chirho = backend_chirho.clone();
        let window_weak_chirho = main_window_chirho.as_weak();
        app_state_chirho.on_load_verse_info_chirho(move |verse_ref_chirho| {
            info!("Loading verse info for: {}", verse_ref_chirho);

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                let backend_ref_chirho = backend_clone_chirho.borrow();

                // Parse reference to extract verse number
                let full_ref_chirho = format!(
                    "{} {}:{}",
                    state_chirho.get_current_book_chirho(),
                    state_chirho.get_current_chapter_chirho(),
                    verse_ref_chirho
                );

                // Get verse text from current verses
                let verses_chirho: Vec<VerseChirho> = state_chirho.get_verses_chirho().iter().collect();
                let verse_text_chirho = verses_chirho
                    .iter()
                    .find(|v_chirho| v_chirho.reference_chirho.as_str() == verse_ref_chirho.as_str())
                    .map(|v_chirho| v_chirho.text_chirho.to_string())
                    .unwrap_or_default();

                // Check highlight status
                let verse_num_chirho: i32 = verse_ref_chirho.parse().unwrap_or(0);
                let highlights_chirho = database_chirho::get_highlights_with_colors_chirho(
                    &backend_ref_chirho.db_conn_chirho,
                    state_chirho.get_current_module_chirho().as_str(),
                    state_chirho.get_current_book_chirho().as_str(),
                    state_chirho.get_current_chapter_chirho(),
                ).unwrap_or_default();
                let highlight_color_chirho = highlights_chirho
                    .iter()
                    .find(|h_chirho| h_chirho.verse_chirho == verse_num_chirho)
                    .map(|h_chirho| h_chirho.color_chirho.clone())
                    .unwrap_or_default();

                // Check if bookmarked
                let is_bookmarked_chirho = database_chirho::is_bookmarked_chirho(
                    &backend_ref_chirho.db_conn_chirho,
                    state_chirho.get_current_module_chirho().as_str(),
                    state_chirho.get_current_book_chirho().as_str(),
                    state_chirho.get_current_chapter_chirho(),
                    verse_num_chirho,
                );

                // Get note if any
                let note_content_chirho = database_chirho::get_note_chirho(
                    &backend_ref_chirho.db_conn_chirho,
                    state_chirho.get_current_module_chirho().as_str(),
                    state_chirho.get_current_book_chirho().as_str(),
                    state_chirho.get_current_chapter_chirho(),
                    verse_num_chirho,
                ).map(|n_chirho| n_chirho.content_chirho).unwrap_or_default();

                // Update info panel state
                state_chirho.set_info_verse_ref_chirho(full_ref_chirho.into());
                state_chirho.set_info_verse_text_chirho(verse_text_chirho.into());
                state_chirho.set_info_has_highlight_chirho(!highlight_color_chirho.is_empty());
                state_chirho.set_info_highlight_color_chirho(highlight_color_chirho.clone().into());
                state_chirho.set_info_has_bookmark_chirho(is_bookmarked_chirho);
                state_chirho.set_info_has_note_chirho(!note_content_chirho.is_empty());
                state_chirho.set_info_note_preview_chirho(note_content_chirho.into());

                // Also open info panel if not already visible
                if !state_chirho.get_info_panel_visible_chirho() {
                    state_chirho.set_info_panel_visible_chirho(true);
                }
            }
        });
    }

    // Statistics loading callback
    {
        let backend_clone_chirho = Rc::clone(&backend_chirho);
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_load_statistics_chirho(move || {
            info!("Loading statistics");

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                let backend_ref_chirho = backend_clone_chirho.borrow();

                match database_chirho::get_statistics_chirho(&backend_ref_chirho.db_conn_chirho) {
                    Ok(stats_chirho) => {
                        state_chirho.set_stats_total_chapters_chirho(stats_chirho.total_chapters_read_chirho);
                        state_chirho.set_stats_unique_chapters_chirho(stats_chirho.unique_chapters_read_chirho);
                        state_chirho.set_stats_reading_time_chirho(
                            database_chirho::format_duration_chirho(stats_chirho.total_reading_time_seconds_chirho).into()
                        );
                        state_chirho.set_stats_current_streak_chirho(stats_chirho.current_streak_chirho);
                        state_chirho.set_stats_longest_streak_chirho(stats_chirho.longest_streak_chirho);
                        state_chirho.set_stats_highlight_count_chirho(stats_chirho.highlight_count_chirho);
                        state_chirho.set_stats_note_count_chirho(stats_chirho.note_count_chirho);
                        state_chirho.set_stats_bookmark_count_chirho(stats_chirho.bookmark_count_chirho);
                        state_chirho.set_stats_books_started_chirho(stats_chirho.books_started_chirho);
                    }
                    Err(e_chirho) => {
                        warn!("Failed to load statistics: {}", e_chirho);
                    }
                }
            }
        });
    }

    // Journal callbacks
    {
        let backend_clone_chirho = Rc::clone(&backend_chirho);
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_load_journal_entries_chirho(move || {
            info!("Loading journal entries");

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                let backend_ref_chirho = backend_clone_chirho.borrow();

                match database_chirho::get_all_journal_entries_chirho(&backend_ref_chirho.db_conn_chirho) {
                    Ok(entries_chirho) => {
                        let display_entries_chirho: Vec<JournalEntryDisplayChirho> = entries_chirho
                            .iter()
                            .map(|e_chirho| JournalEntryDisplayChirho {
                                id_chirho: e_chirho.id_chirho as i32,
                                title_chirho: if e_chirho.title_chirho.is_empty() {
                                    "Untitled".into()
                                } else {
                                    e_chirho.title_chirho.clone().into()
                                },
                                preview_chirho: if e_chirho.content_chirho.len() > 100 {
                                    format!("{}...", &e_chirho.content_chirho[..100]).into()
                                } else {
                                    e_chirho.content_chirho.clone().into()
                                },
                                verse_ref_chirho: e_chirho.verse_ref_chirho.clone().unwrap_or_default().into(),
                                tags_chirho: e_chirho.tags_chirho.clone().unwrap_or_default().into(),
                                date_chirho: format_timestamp_chirho(&e_chirho.created_at_chirho).into(),
                            })
                            .collect();

                        state_chirho.set_journal_entries_chirho(
                            Rc::new(slint::VecModel::from(display_entries_chirho)).into()
                        );
                    }
                    Err(e_chirho) => {
                        warn!("Failed to load journal entries: {}", e_chirho);
                    }
                }
            }
        });
    }

    {
        let backend_clone_chirho = Rc::clone(&backend_chirho);
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_save_journal_entry_chirho(move |id_chirho, title_chirho, content_chirho, verse_ref_chirho, tags_chirho| {
            info!("Saving journal entry: id={}", id_chirho);

            let backend_ref_chirho = backend_clone_chirho.borrow();
            let title_str_chirho = title_chirho.to_string();
            let content_str_chirho = content_chirho.to_string();
            let verse_ref_opt_chirho = if verse_ref_chirho.is_empty() { None } else { Some(verse_ref_chirho.to_string()) };
            let tags_opt_chirho = if tags_chirho.is_empty() { None } else { Some(tags_chirho.to_string()) };

            let result_chirho = if id_chirho < 0 {
                // Create new entry
                database_chirho::create_journal_entry_chirho(
                    &backend_ref_chirho.db_conn_chirho,
                    &title_str_chirho,
                    &content_str_chirho,
                    verse_ref_opt_chirho.as_deref(),
                    tags_opt_chirho.as_deref(),
                )
            } else {
                // Update existing entry
                database_chirho::update_journal_entry_chirho(
                    &backend_ref_chirho.db_conn_chirho,
                    id_chirho as i64,
                    &title_str_chirho,
                    &content_str_chirho,
                    verse_ref_opt_chirho.as_deref(),
                    tags_opt_chirho.as_deref(),
                ).map(|_| id_chirho as i64)
            };

            match result_chirho {
                Ok(_) => {
                    // Reload entries
                    if let Some(window_chirho) = window_weak_chirho.upgrade() {
                        let state_chirho = window_chirho.global::<AppStateChirho>();
                        state_chirho.invoke_load_journal_entries_chirho();
                        state_chirho.set_status_message_chirho("Journal entry saved".into());
                    }
                }
                Err(e_chirho) => {
                    warn!("Failed to save journal entry: {}", e_chirho);
                }
            }
        });
    }

    {
        let backend_clone_chirho = Rc::clone(&backend_chirho);
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_delete_journal_entry_chirho(move |id_chirho| {
            info!("Deleting journal entry: {}", id_chirho);

            let backend_ref_chirho = backend_clone_chirho.borrow();

            match database_chirho::delete_journal_entry_chirho(&backend_ref_chirho.db_conn_chirho, id_chirho as i64) {
                Ok(_) => {
                    // Reload entries
                    if let Some(window_chirho) = window_weak_chirho.upgrade() {
                        let state_chirho = window_chirho.global::<AppStateChirho>();
                        state_chirho.invoke_load_journal_entries_chirho();
                        state_chirho.set_status_message_chirho("Journal entry deleted".into());
                    }
                }
                Err(e_chirho) => {
                    warn!("Failed to delete journal entry: {}", e_chirho);
                }
            }
        });
    }

    // Prayer requests callbacks
    {
        let backend_clone_chirho = Rc::clone(&backend_chirho);
        let window_weak_chirho = main_window_chirho.as_weak();
        app_state_chirho.on_load_prayer_requests_chirho(move || {
            info!("Loading prayer requests");

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                let backend_ref_chirho = backend_clone_chirho.borrow();

                match database_chirho::get_all_prayer_requests_chirho(&backend_ref_chirho.db_conn_chirho) {
                    Ok(requests_chirho) => {
                        let display_requests_chirho: Vec<PrayerRequestDisplayChirho> = requests_chirho
                            .iter()
                            .map(|r_chirho| PrayerRequestDisplayChirho {
                                id_chirho: r_chirho.id_chirho as i32,
                                title_chirho: r_chirho.title_chirho.clone().into(),
                                description_chirho: r_chirho.description_chirho.clone().unwrap_or_default().into(),
                                verse_ref_chirho: r_chirho.verse_ref_chirho.clone().unwrap_or_default().into(),
                                category_chirho: r_chirho.category_chirho.clone().unwrap_or_default().into(),
                                is_answered_chirho: r_chirho.is_answered_chirho,
                                date_chirho: format_timestamp_chirho(&r_chirho.created_at_chirho).into(),
                            })
                            .collect();

                        state_chirho.set_prayer_requests_chirho(
                            Rc::new(slint::VecModel::from(display_requests_chirho)).into()
                        );
                    }
                    Err(e_chirho) => {
                        warn!("Failed to load prayer requests: {}", e_chirho);
                    }
                }
            }
        });
    }

    {
        let backend_clone_chirho = Rc::clone(&backend_chirho);
        let window_weak_chirho = main_window_chirho.as_weak();
        app_state_chirho.on_save_prayer_request_chirho(move |title_chirho, description_chirho, verse_ref_chirho, category_chirho| {
            info!("Saving prayer request: {}", title_chirho.as_str());

            let backend_ref_chirho = backend_clone_chirho.borrow();

            let description_opt_chirho = if description_chirho.is_empty() { None } else { Some(description_chirho.to_string()) };
            let verse_ref_opt_chirho = if verse_ref_chirho.is_empty() { None } else { Some(verse_ref_chirho.to_string()) };
            let category_opt_chirho = if category_chirho.is_empty() { None } else { Some(category_chirho.to_string()) };

            match database_chirho::create_prayer_request_chirho(
                &backend_ref_chirho.db_conn_chirho,
                title_chirho.as_str(),
                description_opt_chirho.as_deref(),
                verse_ref_opt_chirho.as_deref(),
                category_opt_chirho.as_deref(),
            ) {
                Ok(_) => {
                    if let Some(window_chirho) = window_weak_chirho.upgrade() {
                        let state_chirho = window_chirho.global::<AppStateChirho>();
                        state_chirho.invoke_load_prayer_requests_chirho();
                        state_chirho.set_status_message_chirho("Prayer request saved".into());
                    }
                }
                Err(e_chirho) => {
                    warn!("Failed to save prayer request: {}", e_chirho);
                }
            }
        });
    }

    {
        let backend_clone_chirho = Rc::clone(&backend_chirho);
        let window_weak_chirho = main_window_chirho.as_weak();
        app_state_chirho.on_mark_prayer_answered_chirho(move |id_chirho| {
            info!("Marking prayer as answered: {}", id_chirho);

            let backend_ref_chirho = backend_clone_chirho.borrow();

            match database_chirho::mark_prayer_answered_chirho(&backend_ref_chirho.db_conn_chirho, id_chirho as i64) {
                Ok(_) => {
                    if let Some(window_chirho) = window_weak_chirho.upgrade() {
                        let state_chirho = window_chirho.global::<AppStateChirho>();
                        state_chirho.invoke_load_prayer_requests_chirho();
                        state_chirho.set_status_message_chirho("Prayer marked as answered! 🙏".into());
                    }
                }
                Err(e_chirho) => {
                    warn!("Failed to mark prayer as answered: {}", e_chirho);
                }
            }
        });
    }

    {
        let backend_clone_chirho = Rc::clone(&backend_chirho);
        let window_weak_chirho = main_window_chirho.as_weak();
        app_state_chirho.on_delete_prayer_request_chirho(move |id_chirho| {
            info!("Deleting prayer request: {}", id_chirho);

            let backend_ref_chirho = backend_clone_chirho.borrow();

            match database_chirho::delete_prayer_request_chirho(&backend_ref_chirho.db_conn_chirho, id_chirho as i64) {
                Ok(_) => {
                    if let Some(window_chirho) = window_weak_chirho.upgrade() {
                        let state_chirho = window_chirho.global::<AppStateChirho>();
                        state_chirho.invoke_load_prayer_requests_chirho();
                        state_chirho.set_status_message_chirho("Prayer request deleted".into());
                    }
                }
                Err(e_chirho) => {
                    warn!("Failed to delete prayer request: {}", e_chirho);
                }
            }
        });
    }

    // Verse of the Day callbacks
    {
        let window_weak_chirho = main_window_chirho.as_weak();
        app_state_chirho.on_load_verse_of_the_day_chirho(move || {
            info!("Loading verse of the day");

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();

                // Get the verse of the day based on day of year
                let (reference_chirho, text_chirho) = get_verse_of_the_day_chirho();

                // Format today's date
                let now_chirho = std::time::SystemTime::now();
                let since_epoch_chirho = now_chirho.duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
                let days_chirho = (since_epoch_chirho.as_secs() / 86400) as u32;
                // Simple date approximation
                let date_str_chirho = format_date_chirho(days_chirho);

                state_chirho.set_votd_reference_chirho(reference_chirho.into());
                state_chirho.set_votd_text_chirho(text_chirho.into());
                state_chirho.set_votd_date_chirho(date_str_chirho.into());
            }
        });

        let backend_clone_chirho = Rc::clone(&backend_chirho);
        let window_weak_chirho = main_window_chirho.as_weak();
        app_state_chirho.on_goto_votd_chirho(move || {
            info!("Navigating to verse of the day");

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                let reference_chirho = state_chirho.get_votd_reference_chirho().to_string();

                // Parse the reference and navigate
                if let Some((book_chirho, chapter_chirho, _verse_chirho)) = parse_reference_chirho(&reference_chirho) {
                    let mut backend_ref_chirho = backend_clone_chirho.borrow_mut();
                    backend_ref_chirho.navigate_to_chirho(&book_chirho, chapter_chirho);

                    // Update UI state
                    state_chirho.set_current_book_chirho(book_chirho.clone().into());
                    state_chirho.set_current_chapter_chirho(chapter_chirho);

                    // Update chapter count
                    let chapter_count_chirho = BIBLE_BOOKS_CHIRHO
                        .iter()
                        .find(|(name_chirho, _)| *name_chirho == book_chirho.as_str())
                        .map(|(_, count_chirho)| *count_chirho)
                        .unwrap_or(1);
                    state_chirho.set_current_book_chapter_count_chirho(chapter_count_chirho);

                    // Load and display verses
                    let verses_chirho = backend_ref_chirho.get_verses_chirho();
                    state_chirho.set_verses_chirho(Rc::new(slint::VecModel::from(verses_chirho)).into());

                    state_chirho.set_status_message_chirho(
                        format!("Navigated to {}", reference_chirho).into()
                    );
                }
            }
        });
    }

    // Strong's Numbers callbacks
    {
        let window_weak_chirho = main_window_chirho.as_weak();

        // Toggle Strong's display
        app_state_chirho.on_toggle_strongs_display_chirho(move || {
            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                let current_chirho = state_chirho.get_strongs_visible_chirho();
                state_chirho.set_strongs_visible_chirho(!current_chirho);
                let msg_chirho = if !current_chirho {
                    "Strong's Numbers display enabled"
                } else {
                    "Strong's Numbers display disabled"
                };
                state_chirho.set_status_message_chirho(msg_chirho.into());
                info!("{}", msg_chirho);
            }
        });
    }

    {
        let window_weak_chirho = main_window_chirho.as_weak();

        // Look up a Strong's number
        app_state_chirho.on_lookup_strongs_chirho(move |number_chirho| {
            info!("Looking up Strong's number: {}", number_chirho);

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();

                // Get the Strong's definition (in production, from a lexicon module)
                let entry_chirho = get_strongs_definition_chirho(&number_chirho);

                state_chirho.set_strongs_current_entry_chirho(entry_chirho);
                state_chirho.set_strongs_popup_visible_chirho(true);
            }
        });
    }

    // Morphology callbacks
    {
        let window_weak_chirho = main_window_chirho.as_weak();

        // Toggle Morphology display
        app_state_chirho.on_toggle_morphology_display_chirho(move || {
            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                let current_chirho = state_chirho.get_morphology_visible_chirho();
                state_chirho.set_morphology_visible_chirho(!current_chirho);
                let msg_chirho = if !current_chirho {
                    "Morphology display enabled"
                } else {
                    "Morphology display disabled"
                };
                state_chirho.set_status_message_chirho(msg_chirho.into());
                info!("{}", msg_chirho);
            }
        });
    }

    {
        let window_weak_chirho = main_window_chirho.as_weak();

        // Look up a morphology code
        app_state_chirho.on_lookup_morphology_chirho(move |code_chirho| {
            info!("Looking up morphology code: {}", code_chirho);

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();

                // Parse the morphology code
                let (parsed_chirho, is_hebrew_chirho) = parse_morphology_code_chirho(&code_chirho);

                state_chirho.set_morphology_code_chirho(code_chirho.clone());
                state_chirho.set_morphology_parsed_chirho(parsed_chirho.into());
                state_chirho.set_morphology_is_hebrew_chirho(is_hebrew_chirho);
                state_chirho.set_morphology_popup_visible_chirho(true);
            }
        });
    }

    // Cross-References callbacks
    {
        let window_weak_chirho = main_window_chirho.as_weak();

        // Toggle Cross-References display
        app_state_chirho.on_toggle_cross_refs_display_chirho(move || {
            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                let current_chirho = state_chirho.get_cross_refs_visible_chirho();
                state_chirho.set_cross_refs_visible_chirho(!current_chirho);
                let msg_chirho = if !current_chirho {
                    "Cross-references display enabled"
                } else {
                    "Cross-references display disabled"
                };
                state_chirho.set_status_message_chirho(msg_chirho.into());
                info!("{}", msg_chirho);
            }
        });
    }

    {
        let window_weak_chirho = main_window_chirho.as_weak();

        // Load cross-references for a verse
        app_state_chirho.on_load_cross_refs_chirho(move |verse_ref_chirho| {
            info!("Loading cross-references for: {}", verse_ref_chirho);

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();

                // Get cross-references (sample data - in production from TSK module)
                let refs_chirho = get_sample_cross_refs_chirho(&verse_ref_chirho);

                state_chirho.set_cross_refs_verse_chirho(verse_ref_chirho.clone());
                state_chirho.set_cross_refs_list_chirho(Rc::new(slint::VecModel::from(refs_chirho)).into());
                state_chirho.set_cross_refs_popup_visible_chirho(true);
            }
        });
    }

    // Footnotes callbacks
    {
        let window_weak_chirho = main_window_chirho.as_weak();

        // Toggle Footnotes display
        app_state_chirho.on_toggle_footnotes_display_chirho(move || {
            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                let current_chirho = state_chirho.get_footnotes_visible_chirho();
                state_chirho.set_footnotes_visible_chirho(!current_chirho);
                let msg_chirho = if !current_chirho {
                    "Footnotes display enabled"
                } else {
                    "Footnotes display disabled"
                };
                state_chirho.set_status_message_chirho(msg_chirho.into());
                info!("{}", msg_chirho);
            }
        });
    }

    {
        let window_weak_chirho = main_window_chirho.as_weak();

        // Look up a footnote
        app_state_chirho.on_lookup_footnote_chirho(move |verse_ref_chirho, marker_chirho| {
            info!("Looking up footnote {} for: {}", marker_chirho, verse_ref_chirho);

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();

                // Get footnote content (sample data - in production from module)
                let (content_chirho, type_chirho) = get_sample_footnote_chirho(&verse_ref_chirho, &marker_chirho);

                state_chirho.set_footnote_marker_chirho(marker_chirho.clone());
                state_chirho.set_footnote_content_chirho(content_chirho.into());
                state_chirho.set_footnote_type_chirho(type_chirho.into());
                state_chirho.set_footnote_popup_visible_chirho(true);
            }
        });
    }

    // Reading Plans callbacks
    {
        let window_weak_chirho = main_window_chirho.as_weak();

        // Load reading plans
        app_state_chirho.on_load_reading_plans_chirho(move || {
            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();

                // Pre-built reading plans
                let plans_chirho = get_builtin_reading_plans_chirho();
                state_chirho.set_reading_plans_chirho(Rc::new(slint::VecModel::from(plans_chirho)).into());

                // Set today's reading if there's an active plan
                let today_reading_chirho = get_todays_reading_chirho();
                state_chirho.set_today_reading_chirho(today_reading_chirho);

                info!("Loaded reading plans");
            }
        });
    }

    {
        let window_weak_chirho = main_window_chirho.as_weak();

        // Start a reading plan
        app_state_chirho.on_start_reading_plan_chirho(move |plan_id_chirho| {
            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();

                info!("Starting reading plan {}", plan_id_chirho);

                // Get the plan and mark it active
                let plans_chirho = get_builtin_reading_plans_chirho();
                if let Some(mut plan_chirho) = plans_chirho.into_iter().find(|p_chirho| p_chirho.id_chirho == plan_id_chirho) {
                    plan_chirho.is_active_chirho = true;
                    state_chirho.set_active_plan_chirho(plan_chirho);
                }

                // Set today's reading
                let today_reading_chirho = get_todays_reading_chirho();
                state_chirho.set_today_reading_chirho(today_reading_chirho);

                state_chirho.set_status_message_chirho("Reading plan started!".into());
            }
        });
    }

    {
        let window_weak_chirho = main_window_chirho.as_weak();

        // Mark a day as complete
        app_state_chirho.on_mark_day_complete_chirho(move |_plan_id_chirho, day_number_chirho| {
            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();

                info!("Marking day {} as complete", day_number_chirho);

                // Update the reading day
                let mut today_chirho = state_chirho.get_today_reading_chirho();
                today_chirho.is_completed_chirho = true;
                state_chirho.set_today_reading_chirho(today_chirho);

                // Update active plan progress
                let mut plan_chirho = state_chirho.get_active_plan_chirho();
                plan_chirho.current_day_chirho = day_number_chirho + 1;
                plan_chirho.progress_percent_chirho = (day_number_chirho * 100 / plan_chirho.total_days_chirho).min(100);
                state_chirho.set_active_plan_chirho(plan_chirho);

                state_chirho.set_status_message_chirho(format!("Day {} complete! 🎉", day_number_chirho).into());
            }
        });
    }

    {
        let window_weak_chirho = main_window_chirho.as_weak();
        let backend_clone_chirho = backend_chirho.clone();

        // Navigate to a reading reference
        app_state_chirho.on_navigate_to_reading_chirho(move |reading_chirho| {
            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();

                // Parse the first reference from the reading
                let first_ref_chirho = reading_chirho.split(',').next().unwrap_or(&reading_chirho);

                if let Some((book_chirho, chapter_chirho, _)) = parse_reference_chirho(first_ref_chirho.trim()) {
                    let mut backend_ref_chirho = backend_clone_chirho.borrow_mut();
                    backend_ref_chirho.navigate_to_chirho(&book_chirho, chapter_chirho);

                    // Update UI state
                    state_chirho.set_current_book_chirho(book_chirho.clone().into());
                    state_chirho.set_current_chapter_chirho(chapter_chirho);

                    let chapter_count_chirho = BIBLE_BOOKS_CHIRHO
                        .iter()
                        .find(|(name_chirho, _)| *name_chirho == book_chirho.as_str())
                        .map(|(_, count_chirho)| *count_chirho)
                        .unwrap_or(1);
                    state_chirho.set_current_book_chapter_count_chirho(chapter_count_chirho);

                    let verses_chirho = backend_ref_chirho.get_verses_chirho();
                    state_chirho.set_verses_chirho(Rc::new(slint::VecModel::from(verses_chirho)).into());

                    state_chirho.set_status_message_chirho(format!("Reading: {}", reading_chirho).into());

                    info!("Navigating to reading: {}", reading_chirho);
                }
            }
        });
    }

    // Commentary callbacks
    {
        let window_weak_chirho = main_window_chirho.as_weak();

        // Load commentary for a verse
        app_state_chirho.on_load_commentary_chirho(move |verse_ref_chirho| {
            info!("Loading commentary for: {}", verse_ref_chirho);

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();

                // Get sample commentary content
                let (content_chirho, module_chirho) = get_sample_commentary_chirho(&verse_ref_chirho);

                state_chirho.set_commentary_verse_ref_chirho(verse_ref_chirho.clone());
                state_chirho.set_commentary_content_chirho(content_chirho.into());
                state_chirho.set_commentary_module_chirho(module_chirho.into());

                // Set available commentary modules (sample list)
                let modules_chirho: Vec<slint::SharedString> = vec![
                    "MHCC".into(), "Gill".into(), "Barnes".into(), "Clarke".into()
                ];
                state_chirho.set_commentary_modules_chirho(Rc::new(slint::VecModel::from(modules_chirho)).into());
            }
        });
    }

    {
        let window_weak_chirho = main_window_chirho.as_weak();

        // Select commentary module
        app_state_chirho.on_select_commentary_module_chirho(move |module_chirho| {
            info!("Selected commentary module: {}", module_chirho);

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                state_chirho.set_commentary_module_chirho(module_chirho.clone());

                // Reload commentary for current verse with new module
                let verse_ref_chirho = state_chirho.get_commentary_verse_ref_chirho();
                let (content_chirho, _) = get_sample_commentary_chirho(&verse_ref_chirho);
                state_chirho.set_commentary_content_chirho(content_chirho.into());
            }
        });
    }

    // Lexicon callbacks
    {
        let window_weak_chirho = main_window_chirho.as_weak();

        // Search lexicon
        app_state_chirho.on_search_lexicon_chirho(move |query_chirho| {
            info!("Searching lexicon for: {}", query_chirho);

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();

                // Get sample lexicon entry
                let (key_chirho, content_chirho) = get_sample_lexicon_entry_chirho(&query_chirho);

                state_chirho.set_lexicon_current_entry_chirho(key_chirho.into());
                state_chirho.set_lexicon_entry_content_chirho(content_chirho.into());

                // Add to history (max 5 entries)
                let mut history_chirho: Vec<slint::SharedString> = state_chirho.get_lexicon_history_chirho()
                    .iter()
                    .collect();
                if !history_chirho.iter().any(|h_chirho| h_chirho.as_str() == query_chirho.as_str()) {
                    history_chirho.insert(0, query_chirho.clone());
                    if history_chirho.len() > 5 {
                        history_chirho.truncate(5);
                    }
                    state_chirho.set_lexicon_history_chirho(Rc::new(slint::VecModel::from(history_chirho)).into());
                }
            }
        });
    }

    {
        let window_weak_chirho = main_window_chirho.as_weak();

        // Browse lexicon by letter
        app_state_chirho.on_browse_lexicon_chirho(move |letter_chirho| {
            info!("Browsing lexicon for letter: {}", letter_chirho);

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();

                // Get sample entries starting with letter
                let content_chirho = format!(
                    "Entries starting with '{}'\n\nInstall a lexicon module (like Strong's Dictionary) to browse entries alphabetically.",
                    letter_chirho
                );

                state_chirho.set_lexicon_current_entry_chirho(format!("{}...", letter_chirho).into());
                state_chirho.set_lexicon_entry_content_chirho(content_chirho.into());
            }
        });
    }

    {
        let window_weak_chirho = main_window_chirho.as_weak();

        // Select lexicon module
        app_state_chirho.on_select_lexicon_module_chirho(move |module_chirho| {
            info!("Selected lexicon module: {}", module_chirho);

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                state_chirho.set_lexicon_module_chirho(module_chirho);
            }
        });
    }

    // Set versification system callback
    {
        let backend_clone_chirho = Rc::clone(&backend_chirho);
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_set_versification_chirho(move |index_chirho| {
            let versification_names_chirho = ["KJV", "Catholic", "Orthodox", "LXX", "Vulgate", "Luther"];
            let name_chirho = versification_names_chirho.get(index_chirho as usize).unwrap_or(&"KJV");
            info!("Set versification system to: {} (index {})", name_chirho, index_chirho);

            // Save versification setting to database
            let backend_ref_chirho = backend_clone_chirho.borrow();
            if let Err(e_chirho) = database_chirho::set_setting_chirho(
                &backend_ref_chirho.db_conn_chirho,
                "versification_system",
                &index_chirho.to_string(),
            ) {
                error!("Failed to save versification setting: {}", e_chirho);
            }

            // Update status message
            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                state_chirho.set_status_message_chirho(
                    format!("Versification set to {}", name_chirho).into()
                );
            }
        });
    }

    // Load saved versification setting
    {
        let backend_ref_chirho = backend_chirho.borrow();
        if let Some(vers_index_str_chirho) = database_chirho::get_setting_chirho(
            &backend_ref_chirho.db_conn_chirho,
            "versification_system",
        ) {
            if let Ok(index_chirho) = vers_index_str_chirho.parse::<i32>() {
                app_state_chirho.set_versification_index_chirho(index_chirho);
                info!("Loaded versification setting: index {}", index_chirho);
            }
        }
    }

    // Interlinear display callbacks (CLX-051)
    {
        let backend_clone_chirho = backend_chirho.clone();
        let window_weak_chirho = main_window_chirho.as_weak();

        // Toggle interlinear display
        app_state_chirho.on_toggle_interlinear_display_chirho(move || {
            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                let visible_chirho = !state_chirho.get_interlinear_visible_chirho();
                state_chirho.set_interlinear_visible_chirho(visible_chirho);

                if visible_chirho {
                    // Load interlinear for current verse
                    let book_chirho = state_chirho.get_current_book_chirho();
                    let chapter_chirho = state_chirho.get_current_chapter_chirho();
                    let verse_ref_chirho = format!("{} {}:1", book_chirho, chapter_chirho);
                    state_chirho.set_interlinear_verse_ref_chirho(verse_ref_chirho.clone().into());

                    // Try to extract from OSIS, fallback to sample data
                    let backend_ref_chirho = backend_clone_chirho.borrow();
                    let words_chirho = backend_ref_chirho.bible_engine_chirho.extract_interlinear_chirho(&verse_ref_chirho);
                    let words_chirho = if words_chirho.is_empty() {
                        get_sample_interlinear_chirho(&verse_ref_chirho)
                    } else {
                        words_chirho
                    };
                    let words_model_chirho: Rc<slint::VecModel<InterlinearWordChirho>> =
                        Rc::new(slint::VecModel::from(words_chirho));
                    state_chirho.set_interlinear_words_chirho(slint::ModelRc::from(words_model_chirho));

                    state_chirho.set_status_message_chirho("Interlinear display enabled".into());
                } else {
                    state_chirho.set_status_message_chirho("Interlinear display disabled".into());
                }
            }
        });
    }

    {
        let backend_clone_chirho = backend_chirho.clone();
        let window_weak_chirho = main_window_chirho.as_weak();

        // Load interlinear for a specific verse
        app_state_chirho.on_load_interlinear_chirho(move |verse_ref_chirho| {
            info!("Loading interlinear for: {}", verse_ref_chirho);

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();
                state_chirho.set_interlinear_verse_ref_chirho(verse_ref_chirho.clone());

                // Try to extract from OSIS, fallback to sample data
                let backend_ref_chirho = backend_clone_chirho.borrow();
                let words_chirho = backend_ref_chirho.bible_engine_chirho.extract_interlinear_chirho(verse_ref_chirho.as_str());
                let words_chirho = if words_chirho.is_empty() {
                    get_sample_interlinear_chirho(verse_ref_chirho.as_str())
                } else {
                    words_chirho
                };
                let words_model_chirho: Rc<slint::VecModel<InterlinearWordChirho>> =
                    Rc::new(slint::VecModel::from(words_chirho));
                state_chirho.set_interlinear_words_chirho(slint::ModelRc::from(words_model_chirho));
            }
        });
    }

    // Check if first run and show onboarding
    {
        let backend_ref_chirho = backend_chirho.borrow();
        let onboarding_complete_chirho = database_chirho::get_setting_chirho(
            &backend_ref_chirho.db_conn_chirho,
            "onboarding_complete",
        );
        if onboarding_complete_chirho.is_none() {
            info!("First run detected, showing onboarding wizard");
            app_state_chirho.set_onboarding_visible_chirho(true);
            app_state_chirho.set_onboarding_step_chirho(0);
        }
    }

    // Set initial status
    app_state_chirho.set_status_message_chirho("Welcome to Codex Lux Chirho - Your Bible Study Companion".into());

    // Run the application
    main_window_chirho.run()
}

/// Get sample commentary content for a verse
/// In production, this would query a commentary module via rsword_chirho
fn get_sample_commentary_chirho(verse_ref_chirho: &str) -> (String, String) {
    // Sample commentary entries for demonstration
    match verse_ref_chirho {
        "Genesis 1:1" | "Genesis 1" => (
            "\"In the beginning\" - This phrase marks the absolute beginning of all created existence. \
            The Hebrew word 'reshiyth' (רֵאשִׁית) indicates the starting point of time itself.\n\n\
            \"God created\" - The Hebrew 'bara' (בָּרָא) is used exclusively of divine activity, \
            implying creation from nothing (ex nihilo). This verb never has a material object.\n\n\
            \"the heaven and the earth\" - A merism (figure of speech using extremes to indicate totality) \
            meaning 'everything' - the entire cosmos and all that is in it.".to_string(),
            "MHCC".to_string()
        ),
        "John 3:16" | "John 3" => (
            "This verse is often called 'the gospel in miniature' as it summarizes the entire message \
            of salvation.\n\n\
            \"For God so loved\" - The Greek 'agape' (ἀγάπη) refers to unconditional, sacrificial love. \
            The word 'so' (Greek: houto) emphasizes the manner and intensity of God's love.\n\n\
            \"the world\" - Greek 'kosmos' (κόσμος) - not just the Jews, but all of humanity, \
            showing the universal scope of God's redemptive plan.\n\n\
            \"only begotten Son\" - Greek 'monogenes' (μονογενής) - meaning unique, one of a kind. \
            It emphasizes Jesus' unique relationship with the Father.".to_string(),
            "MHCC".to_string()
        ),
        "Psalm 23:1" | "Psalm 23" | "Psalms 23:1" | "Psalms 23" => (
            "\"The LORD is my shepherd\" - David, himself a shepherd, uses this intimate metaphor \
            to describe his relationship with God.\n\n\
            The divine name YHWH (יְהוָה) emphasizes the covenant relationship. As shepherd, \
            God provides guidance, protection, and provision.\n\n\
            \"I shall not want\" - Complete trust that all needs will be met. Not a promise of \
            luxury, but of sufficiency in God's care.".to_string(),
            "MHCC".to_string()
        ),
        "Romans 8:28" | "Romans 8" => (
            "\"All things work together for good\" - Not that all things ARE good, but that God \
            orchestrates them for His purposes.\n\n\
            \"to them that love God\" - The promise is conditional on relationship. Those who love \
            God experience His providential care.\n\n\
            \"called according to his purpose\" - Refers to God's sovereign election and \
            the unfolding of His redemptive plan for His people.".to_string(),
            "MHCC".to_string()
        ),
        _ => (
            format!(
                "Commentary for {} not found in sample data.\n\n\
                Install commentary modules (like Matthew Henry's Commentary, Gill's Exposition, \
                Barnes' Notes) to access verse-by-verse explanations.",
                verse_ref_chirho
            ),
            "".to_string()
        ),
    }
}

/// Get sample lexicon entry
/// In production, this would query a lexicon module via rsword_chirho
fn get_sample_lexicon_entry_chirho(query_chirho: &str) -> (String, String) {
    let query_lower_chirho = query_chirho.to_lowercase();

    match query_lower_chirho.as_str() {
        "love" | "agape" | "ἀγάπη" => (
            "ἀγάπη (agape)".to_string(),
            "Greek: ἀγάπη (agape) [ag-ah'-pay]\n\
            Strong's: G26\n\n\
            Definition: Love, benevolence, good will, affection.\n\n\
            Usage in NT: Used 116 times\n\n\
            Meaning:\n\
            1. Brotherly love, affection, good will\n\
            2. Love feasts (meals held by early Christians)\n\
            3. The love of God or Christ for humanity\n\
            4. The love of Christians for God and fellow believers\n\n\
            Unlike 'philos' (friendship love) or 'eros' (romantic love), agape refers to \
            unconditional, sacrificial love that seeks the highest good of the beloved.".to_string()
        ),
        "god" | "theos" | "θεός" => (
            "θεός (theos)".to_string(),
            "Greek: θεός (theos) [theh'-os]\n\
            Strong's: G2316\n\n\
            Definition: A god, deity; the supreme Divinity; God.\n\n\
            Usage in NT: Used 1343 times\n\n\
            Meaning:\n\
            1. God, the Creator and supreme ruler of the universe\n\
            2. Any deity or god (in polytheistic context)\n\
            3. One who possesses divine nature\n\n\
            In the New Testament, theos is used predominantly for the God of Israel, \
            the Father of Jesus Christ.".to_string()
        ),
        "word" | "logos" | "λόγος" => (
            "λόγος (logos)".to_string(),
            "Greek: λόγος (logos) [log'-os]\n\
            Strong's: G3056\n\n\
            Definition: Word, reason, account, discourse.\n\n\
            Usage in NT: Used 330 times\n\n\
            Meaning:\n\
            1. A word (as embodying an idea)\n\
            2. A saying, statement, declaration\n\
            3. Discourse, speech, teaching\n\
            4. Reason, the mental faculty of reasoning\n\
            5. The Word (Christ as the living expression of God)\n\n\
            In John 1:1, logos is used theologically to identify Christ as the \
            pre-existent Word of God, through whom all things were created.".to_string()
        ),
        "faith" | "pistis" | "πίστις" => (
            "πίστις (pistis)".to_string(),
            "Greek: πίστις (pistis) [pis'-tis]\n\
            Strong's: G4102\n\n\
            Definition: Faith, belief, trust, confidence.\n\n\
            Usage in NT: Used 244 times\n\n\
            Meaning:\n\
            1. Conviction of the truth, belief\n\
            2. Trust, confidence, reliance\n\
            3. The content of faith (what is believed)\n\
            4. Faithfulness, reliability, trustworthiness\n\n\
            In Paul's writings, pistis is central to salvation - it is the means by which \
            humans receive God's grace (Ephesians 2:8-9).".to_string()
        ),
        _ => (
            query_chirho.to_string(),
            format!(
                "Entry '{}' not found in sample lexicon data.\n\n\
                Install lexicon modules (like Strong's Dictionary, Thayer's Greek Lexicon, \
                BDB Hebrew Lexicon) to look up word definitions.\n\n\
                You can also search by Strong's number (e.g., 'G26' or 'H430').",
                query_chirho
            )
        ),
    }
}

/// Parse a morphology code and return human-readable explanation
/// In production, this would use proper morphology parsing from rsword_chirho
fn parse_morphology_code_chirho(code_chirho: &str) -> (String, bool) {
    // Determine if Hebrew or Greek based on code patterns
    // Hebrew codes typically start with letters like H, N, V, A, etc. without hyphens
    // Greek Robinson codes use patterns like V-AAI-3S
    let is_hebrew_chirho = !code_chirho.contains('-');

    if is_hebrew_chirho {
        // Hebrew morphology parsing (simplified OSHM-style)
        let mut parts_chirho = Vec::new();

        // Parse each character
        for (i_chirho, c_chirho) in code_chirho.chars().enumerate() {
            match (i_chirho, c_chirho) {
                (0, 'V') => parts_chirho.push("Verb"),
                (0, 'N') => parts_chirho.push("Noun"),
                (0, 'A') => parts_chirho.push("Adjective"),
                (0, 'P') => parts_chirho.push("Preposition"),
                (0, 'C') => parts_chirho.push("Conjunction"),
                (0, 'R') => parts_chirho.push("Pronoun"),
                (0, 'D') => parts_chirho.push("Adverb"),
                (0, 'T') => parts_chirho.push("Particle"),
                (_, 'm') => parts_chirho.push("masculine"),
                (_, 'f') => parts_chirho.push("feminine"),
                (1, 'c') => parts_chirho.push("common"),
                (_, 's') => parts_chirho.push("singular"),
                (_, 'p') => parts_chirho.push("plural"),
                (_, 'd') => parts_chirho.push("dual"),
                (_, 'a') => parts_chirho.push("absolute"),
                (2.., 'c') => parts_chirho.push("construct"),
                _ => {}
            }
        }

        (parts_chirho.join(", "), true)
    } else {
        // Greek Robinson morphology parsing
        let segments_chirho: Vec<&str> = code_chirho.split('-').collect();
        let mut parts_chirho = Vec::new();

        for (i_chirho, seg_chirho) in segments_chirho.iter().enumerate() {
            match (i_chirho, *seg_chirho) {
                // Part of speech
                (0, "V") => parts_chirho.push("Verb"),
                (0, "N") => parts_chirho.push("Noun"),
                (0, "A") => parts_chirho.push("Adjective"),
                (0, "ADV") => parts_chirho.push("Adverb"),
                (0, "P") => parts_chirho.push("Preposition"),
                (0, "C") => parts_chirho.push("Conjunction"),
                (0, "T") => parts_chirho.push("Article"),
                (0, "R") => parts_chirho.push("Relative Pronoun"),
                (0, "D") => parts_chirho.push("Demonstrative"),
                (0, "X") => parts_chirho.push("Indefinite Pronoun"),
                (0, "I") => parts_chirho.push("Interrogative"),
                // Tense/Voice/Mood for verbs
                (1, "AAI") => parts_chirho.push("Aorist Active Indicative"),
                (1, "AAM") => parts_chirho.push("Aorist Active Imperative"),
                (1, "AAP") => parts_chirho.push("Aorist Active Participle"),
                (1, "API") => parts_chirho.push("Aorist Passive Indicative"),
                (1, "PAI") => parts_chirho.push("Present Active Indicative"),
                (1, "PAP") => parts_chirho.push("Present Active Participle"),
                (1, "PPI") => parts_chirho.push("Present Passive Indicative"),
                (1, "PMI") => parts_chirho.push("Present Middle Indicative"),
                (1, "FAI") => parts_chirho.push("Future Active Indicative"),
                (1, "PEI") => parts_chirho.push("Perfect Active Indicative"),
                (1, "RAI") => parts_chirho.push("Perfect Active Indicative"),
                // Person/Number
                (_, "1S") => parts_chirho.push("1st Person Singular"),
                (_, "2S") => parts_chirho.push("2nd Person Singular"),
                (_, "3S") => parts_chirho.push("3rd Person Singular"),
                (_, "1P") => parts_chirho.push("1st Person Plural"),
                (_, "2P") => parts_chirho.push("2nd Person Plural"),
                (_, "3P") => parts_chirho.push("3rd Person Plural"),
                // Case for nouns/adjectives
                (_, "NSM") => parts_chirho.push("Nominative Singular Masculine"),
                (_, "NSF") => parts_chirho.push("Nominative Singular Feminine"),
                (_, "NSN") => parts_chirho.push("Nominative Singular Neuter"),
                (_, "GSM") => parts_chirho.push("Genitive Singular Masculine"),
                (_, "GSF") => parts_chirho.push("Genitive Singular Feminine"),
                (_, "GSN") => parts_chirho.push("Genitive Singular Neuter"),
                (_, "DSM") => parts_chirho.push("Dative Singular Masculine"),
                (_, "DSF") => parts_chirho.push("Dative Singular Feminine"),
                (_, "ASM") => parts_chirho.push("Accusative Singular Masculine"),
                (_, "ASF") => parts_chirho.push("Accusative Singular Feminine"),
                (_, "NPM") => parts_chirho.push("Nominative Plural Masculine"),
                (_, "NPF") => parts_chirho.push("Nominative Plural Feminine"),
                (_, "GPM") => parts_chirho.push("Genitive Plural Masculine"),
                (_, "DPM") => parts_chirho.push("Dative Plural Masculine"),
                (_, "APM") => parts_chirho.push("Accusative Plural Masculine"),
                _ => parts_chirho.push(seg_chirho),
            }
        }

        (parts_chirho.join(", "), false)
    }
}

/// Get sample cross-references for a verse
/// In production, this would query a TSK module via rsword_chirho
fn get_sample_cross_refs_chirho(verse_ref_chirho: &str) -> Vec<slint::SharedString> {
    // Sample cross-references for demonstration
    let refs_chirho: Vec<&str> = match verse_ref_chirho {
        "Genesis 1:1" => vec![
            "John 1:1-3", "Hebrews 11:3", "Psalm 33:6", "Isaiah 40:26", "Colossians 1:16-17"
        ],
        "John 3:16" => vec![
            "Romans 5:8", "1 John 4:9-10", "Romans 8:32", "John 1:14", "John 3:36",
            "1 John 5:11", "Ephesians 2:4-5", "Isaiah 9:6"
        ],
        "Romans 8:28" => vec![
            "Romans 8:35-39", "Jeremiah 29:11", "Genesis 50:20", "Philippians 1:6"
        ],
        "Psalm 23:1" => vec![
            "Isaiah 40:11", "Ezekiel 34:11-12", "John 10:11", "Hebrews 13:20", "1 Peter 2:25"
        ],
        _ => vec![
            "Install TSK module for cross-references"
        ],
    };

    refs_chirho.into_iter().map(|s_chirho| s_chirho.into()).collect()
}

/// Get sample footnote content
/// In production, this would come from the module's footnote markup
fn get_sample_footnote_chirho(verse_ref_chirho: &str, marker_chirho: &str) -> (String, String) {
    // Sample footnotes for demonstration
    match (verse_ref_chirho, marker_chirho) {
        ("Genesis 1:1", "a") => (
            "Or 'When God began to create' or 'In the beginning of God's creating'".to_string(),
            "alternative".to_string()
        ),
        ("Genesis 1:2", "a") => (
            "Hebrew ruach, meaning wind, breath, or spirit".to_string(),
            "translator".to_string()
        ),
        ("John 3:16", "a") => (
            "Or 'only unique Son'; Greek monogenēs (μονογενής)".to_string(),
            "textual".to_string()
        ),
        ("John 1:1", "a") => (
            "Greek Logos (λόγος), meaning Word, reason, or divine expression".to_string(),
            "translator".to_string()
        ),
        ("Matthew 1:23", "a") => (
            "Hebrew 'Immanuel' means 'God with us'".to_string(),
            "explanation".to_string()
        ),
        _ => (
            format!("Footnote {} for {}", marker_chirho, verse_ref_chirho),
            "translator".to_string()
        ),
    }
}

/// Get Strong's definition for a given number (H1234 or G5678)
/// In production, this would query a lexicon module via rsword_chirho
fn get_strongs_definition_chirho(number_chirho: &str) -> StrongsEntryChirho {
    // Sample Strong's definitions for demonstration
    // In production, these would come from installed lexicon modules
    let is_hebrew_chirho = number_chirho.starts_with('H');

    let (lemma_chirho, transliteration_chirho, pronunciation_chirho, definition_chirho, usage_chirho, occurrences_chirho) = match number_chirho {
        // Hebrew Strong's numbers
        "H1" => ("אָב", "av", "awb", "Father, ancestor, originator, patron", "father, ancestor, forefather", 1212),
        "H430" => ("אֱלֹהִים", "elohim", "el-o-heem'", "God, gods, judges, mighty ones", "Used of the true God, false gods, and supernatural beings", 2606),
        "H3068" => ("יְהוָה", "Yᵉhōvâh", "yeh-ho-vaw'", "The LORD, the proper name of the God of Israel", "The tetragrammaton, the covenant name of God", 6519),
        "H7225" => ("רֵאשִׁית", "reshiyth", "ray-sheeth'", "Beginning, first, chief", "beginning, first, firstfruits, best", 51),
        "H776" => ("אֶרֶץ", "erets", "eh'-rets", "Earth, land, ground, country", "earth, land, territory, country, ground", 2504),
        "H8064" => ("שָׁמַיִם", "shamayim", "shaw-mah'-yim", "Heavens, sky", "heaven, heavens, sky, visible heavens", 421),

        // Greek Strong's numbers
        "G1" => ("Α", "alpha", "al'-fah", "Alpha, the first letter of the Greek alphabet", "Used symbolically for 'first' or 'beginning'", 4),
        "G26" => ("ἀγάπη", "agape", "ag-ah'-pay", "Love, affection, benevolence", "love, benevolence, good will, esteem; God's love", 116),
        "G2316" => ("θεός", "theos", "theh'-os", "God, a deity", "a god, deity, the supreme Divinity; God", 1343),
        "G2424" => ("Ἰησοῦς", "Iesous", "ee-ay-sooce'", "Jesus, Joshua", "Jesus, the Son of God; Joshua", 975),
        "G3056" => ("λόγος", "logos", "log'-os", "Word, reason, account", "word, saying, account, matter; the Word (Christ)", 330),
        "G4102" => ("πίστις", "pistis", "pis'-tis", "Faith, belief, trust, confidence", "faith, belief, trust, confidence, fidelity", 244),
        "G5547" => ("Χριστός", "Christos", "khris-tos'", "Christ, anointed one, Messiah", "Christ, the Anointed One, the Messiah", 569),

        _ => ("", "", "", "Strong's number not found in sample data", "Install a lexicon module for full definitions", 0),
    };

    StrongsEntryChirho {
        number_chirho: number_chirho.into(),
        is_hebrew_chirho,
        lemma_chirho: lemma_chirho.into(),
        transliteration_chirho: transliteration_chirho.into(),
        pronunciation_chirho: pronunciation_chirho.into(),
        definition_chirho: definition_chirho.into(),
        usage_chirho: usage_chirho.into(),
        occurrences_chirho,
    }
}

/// Get built-in reading plans
fn get_builtin_reading_plans_chirho() -> Vec<ReadingPlanChirho> {
    vec![
        ReadingPlanChirho {
            id_chirho: 1,
            name_chirho: "Bible in a Year".into(),
            description_chirho: "Read through the entire Bible in 365 days".into(),
            plan_type_chirho: "daily".into(),
            total_days_chirho: 365,
            current_day_chirho: 1,
            is_active_chirho: false,
            progress_percent_chirho: 0,
        },
        ReadingPlanChirho {
            id_chirho: 2,
            name_chirho: "New Testament (90 Days)".into(),
            description_chirho: "Read the New Testament in 90 days".into(),
            plan_type_chirho: "daily".into(),
            total_days_chirho: 90,
            current_day_chirho: 1,
            is_active_chirho: false,
            progress_percent_chirho: 0,
        },
        ReadingPlanChirho {
            id_chirho: 3,
            name_chirho: "Psalms & Proverbs (30 Days)".into(),
            description_chirho: "Daily wisdom from Psalms and Proverbs".into(),
            plan_type_chirho: "daily".into(),
            total_days_chirho: 30,
            current_day_chirho: 1,
            is_active_chirho: false,
            progress_percent_chirho: 0,
        },
        ReadingPlanChirho {
            id_chirho: 4,
            name_chirho: "Gospels (30 Days)".into(),
            description_chirho: "Walk with Jesus through the four Gospels".into(),
            plan_type_chirho: "daily".into(),
            total_days_chirho: 30,
            current_day_chirho: 1,
            is_active_chirho: false,
            progress_percent_chirho: 0,
        },
        ReadingPlanChirho {
            id_chirho: 5,
            name_chirho: "Chronological Bible".into(),
            description_chirho: "Read the Bible in historical order".into(),
            plan_type_chirho: "chronological".into(),
            total_days_chirho: 365,
            current_day_chirho: 1,
            is_active_chirho: false,
            progress_percent_chirho: 0,
        },
    ]
}

/// Get today's reading assignment based on day of year
fn get_todays_reading_chirho() -> ReadingDayChirho {
    use chrono::Datelike;
    let today_chirho = chrono::Local::now();
    let day_of_year_chirho = today_chirho.ordinal() as i32;

    // Sample readings for demonstration (rotating through common passages)
    let readings_chirho = match day_of_year_chirho % 7 {
        0 => "Genesis 1-3",
        1 => "Psalm 23, Proverbs 1",
        2 => "Matthew 5-7",
        3 => "John 1-3",
        4 => "Romans 8, 12",
        5 => "Isaiah 40, 53",
        _ => "Psalm 119:1-48",
    };

    ReadingDayChirho {
        day_number_chirho: day_of_year_chirho,
        readings_chirho: readings_chirho.into(),
        is_completed_chirho: false,
        is_today_chirho: true,
    }
}

/// Format timestamp for display (e.g., "Today 10:30 AM" or "Jan 15")
fn format_timestamp_chirho(timestamp_chirho: &str) -> String {
    // Simple implementation - just show date part
    if let Some(date_part_chirho) = timestamp_chirho.split(' ').next() {
        // Show just the date in a friendly format
        date_part_chirho.to_string()
    } else {
        timestamp_chirho.to_string()
    }
}

/// Format a day count since Unix epoch to a readable date
fn format_date_chirho(days_since_epoch_chirho: u32) -> String {
    // Calculate approximate date from days since epoch (Jan 1, 1970)
    let years_chirho = days_since_epoch_chirho / 365;
    let remaining_days_chirho = days_since_epoch_chirho % 365;
    let year_chirho = 1970 + years_chirho;

    let months_chirho = [
        ("January", 31), ("February", 28), ("March", 31), ("April", 30),
        ("May", 31), ("June", 30), ("July", 31), ("August", 31),
        ("September", 30), ("October", 31), ("November", 30), ("December", 31),
    ];

    let mut day_chirho = remaining_days_chirho;
    let mut month_name_chirho = "January";

    for (name_chirho, days_chirho) in months_chirho {
        if day_chirho <= days_chirho {
            month_name_chirho = name_chirho;
            break;
        }
        day_chirho -= days_chirho;
    }

    format!("{} {}, {}", month_name_chirho, day_chirho.max(1), year_chirho)
}

/// Get the verse of the day based on day of year
/// Returns (reference, text) for a curated set of popular Bible verses
fn get_verse_of_the_day_chirho() -> (String, String) {
    // Curated collection of 365+ beloved Bible verses
    const VOTD_VERSES_CHIRHO: &[(&str, &str)] = &[
        ("John 3:16", "For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life."),
        ("Psalm 23:1", "The LORD is my shepherd; I shall not want."),
        ("Jeremiah 29:11", "For I know the thoughts that I think toward you, saith the LORD, thoughts of peace, and not of evil, to give you an expected end."),
        ("Romans 8:28", "And we know that all things work together for good to them that love God, to them who are the called according to his purpose."),
        ("Philippians 4:13", "I can do all things through Christ which strengtheneth me."),
        ("Proverbs 3:5-6", "Trust in the LORD with all thine heart; and lean not unto thine own understanding. In all thy ways acknowledge him, and he shall direct thy paths."),
        ("Isaiah 40:31", "But they that wait upon the LORD shall renew their strength; they shall mount up with wings as eagles; they shall run, and not be weary; and they shall walk, and not faint."),
        ("Psalm 46:1", "God is our refuge and strength, a very present help in trouble."),
        ("Romans 12:2", "And be not conformed to this world: but be ye transformed by the renewing of your mind, that ye may prove what is that good, and acceptable, and perfect, will of God."),
        ("Matthew 6:33", "But seek ye first the kingdom of God, and his righteousness; and all these things shall be added unto you."),
        ("Psalm 119:105", "Thy word is a lamp unto my feet, and a light unto my path."),
        ("Ephesians 2:8-9", "For by grace are ye saved through faith; and that not of yourselves: it is the gift of God: Not of works, lest any man should boast."),
        ("Joshua 1:9", "Have not I commanded thee? Be strong and of a good courage; be not afraid, neither be thou dismayed: for the LORD thy God is with thee whithersoever thou goest."),
        ("1 Corinthians 13:4-5", "Charity suffereth long, and is kind; charity envieth not; charity vaunteth not itself, is not puffed up, Doth not behave itself unseemly, seeketh not her own, is not easily provoked, thinketh no evil;"),
        ("Psalm 27:1", "The LORD is my light and my salvation; whom shall I fear? the LORD is the strength of my life; of whom shall I be afraid?"),
        ("Galatians 5:22-23", "But the fruit of the Spirit is love, joy, peace, longsuffering, gentleness, goodness, faith, Meekness, temperance: against such there is no law."),
        ("Matthew 11:28", "Come unto me, all ye that labour and are heavy laden, and I will give you rest."),
        ("2 Timothy 1:7", "For God hath not given us the spirit of fear; but of power, and of love, and of a sound mind."),
        ("Hebrews 11:1", "Now faith is the substance of things hoped for, the evidence of things not seen."),
        ("Psalm 37:4", "Delight thyself also in the LORD; and he shall give thee the desires of thine heart."),
        ("Romans 5:8", "But God commendeth his love toward us, in that, while we were yet sinners, Christ died for us."),
        ("2 Corinthians 5:17", "Therefore if any man be in Christ, he is a new creature: old things are passed away; behold, all things are become new."),
        ("Psalm 91:1-2", "He that dwelleth in the secret place of the most High shall abide under the shadow of the Almighty. I will say of the LORD, He is my refuge and my fortress: my God; in him will I trust."),
        ("Isaiah 41:10", "Fear thou not; for I am with thee: be not dismayed; for I am thy God: I will strengthen thee; yea, I will help thee; yea, I will uphold thee with the right hand of my righteousness."),
        ("1 Peter 5:7", "Casting all your care upon him; for he careth for you."),
        ("Psalm 34:8", "O taste and see that the LORD is good: blessed is the man that trusteth in him."),
        ("James 1:2-3", "My brethren, count it all joy when ye fall into divers temptations; Knowing this, that the trying of your faith worketh patience."),
        ("Colossians 3:23", "And whatsoever ye do, do it heartily, as to the Lord, and not unto men;"),
        ("Psalm 103:1", "Bless the LORD, O my soul: and all that is within me, bless his holy name."),
        ("Matthew 5:16", "Let your light so shine before men, that they may see your good works, and glorify your Father which is in heaven."),
        ("Hebrews 12:2", "Looking unto Jesus the author and finisher of our faith; who for the joy that was set before him endured the cross, despising the shame, and is set down at the right hand of the throne of God."),
    ];

    // Get day of year to select verse (cycles through the list)
    let now_chirho = std::time::SystemTime::now();
    let since_epoch_chirho = now_chirho.duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
    let day_of_year_chirho = (since_epoch_chirho.as_secs() / 86400) as usize;

    let index_chirho = day_of_year_chirho % VOTD_VERSES_CHIRHO.len();
    let (reference_chirho, text_chirho) = VOTD_VERSES_CHIRHO[index_chirho];

    (reference_chirho.to_string(), text_chirho.to_string())
}

// ============================================================================
// Bookmark UI Helper
// ============================================================================

/// Load bookmarks from database and convert to UI model
fn load_bookmarks_for_ui_chirho(conn_chirho: &Connection) -> Vec<BookmarkDisplayChirho> {
    match database_chirho::get_all_bookmarks_chirho(conn_chirho) {
        Ok(bookmarks_chirho) => bookmarks_chirho
            .into_iter()
            .map(|bm_chirho| {
                let reference_chirho = format!(
                    "{} {}:{}",
                    bm_chirho.book_chirho,
                    bm_chirho.chapter_chirho,
                    bm_chirho.verse_chirho
                );
                BookmarkDisplayChirho {
                    id_chirho: bm_chirho.id_chirho as i32,
                    reference_chirho: reference_chirho.into(),
                    label_chirho: bm_chirho.label_chirho.unwrap_or_default().into(),
                    book_chirho: bm_chirho.book_chirho.into(),
                    chapter_chirho: bm_chirho.chapter_chirho,
                    verse_chirho: bm_chirho.verse_chirho,
                }
            })
            .collect(),
        Err(e_chirho) => {
            warn!("Failed to load bookmarks: {}", e_chirho);
            Vec::new()
        }
    }
}

/// Load all highlights from database and convert to UI model
fn load_highlights_for_ui_chirho(conn_chirho: &Connection) -> Vec<HighlightDisplayChirho> {
    match database_chirho::get_all_highlights_chirho(conn_chirho) {
        Ok(highlights_chirho) => highlights_chirho
            .into_iter()
            .map(|hl_chirho| {
                let reference_chirho = format!(
                    "{} {}:{}",
                    hl_chirho.book_chirho,
                    hl_chirho.chapter_chirho,
                    hl_chirho.verse_chirho
                );
                // Get a preview of the verse text
                let verses_chirho = get_sample_verses_chirho(&hl_chirho.book_chirho, hl_chirho.chapter_chirho);
                let preview_chirho = verses_chirho
                    .iter()
                    .find(|(num_chirho, _)| num_chirho.parse::<i32>().unwrap_or(0) == hl_chirho.verse_chirho)
                    .map(|(_, text_chirho)| {
                        if text_chirho.len() > 80 {
                            format!("{}...", &text_chirho[..80])
                        } else {
                            text_chirho.clone()
                        }
                    })
                    .unwrap_or_default();
                HighlightDisplayChirho {
                    id_chirho: hl_chirho.id_chirho as i32,
                    reference_chirho: reference_chirho.into(),
                    color_chirho: hl_chirho.color_chirho.into(),
                    book_chirho: hl_chirho.book_chirho.into(),
                    chapter_chirho: hl_chirho.chapter_chirho,
                    verse_chirho: hl_chirho.verse_chirho,
                    preview_chirho: preview_chirho.into(),
                }
            })
            .collect(),
        Err(e_chirho) => {
            warn!("Failed to load highlights: {}", e_chirho);
            Vec::new()
        }
    }
}

/// Load all notes from database and convert to UI model
fn load_notes_for_ui_chirho(conn_chirho: &Connection) -> Vec<NoteDisplayChirho> {
    match database_chirho::get_all_notes_chirho(conn_chirho) {
        Ok(notes_chirho) => notes_chirho
            .into_iter()
            .map(|note_chirho| {
                let reference_chirho = format!(
                    "{} {}:{}",
                    note_chirho.book_chirho,
                    note_chirho.chapter_chirho,
                    note_chirho.verse_chirho
                );
                // Create a preview of the note content
                let preview_chirho = if note_chirho.content_chirho.len() > 100 {
                    format!("{}...", &note_chirho.content_chirho[..100])
                } else {
                    note_chirho.content_chirho.clone()
                };
                NoteDisplayChirho {
                    id_chirho: note_chirho.id_chirho as i32,
                    reference_chirho: reference_chirho.into(),
                    book_chirho: note_chirho.book_chirho.into(),
                    chapter_chirho: note_chirho.chapter_chirho,
                    verse_chirho: note_chirho.verse_chirho,
                    preview_chirho: preview_chirho.into(),
                    updated_at_chirho: format_timestamp_chirho(&note_chirho.updated_at_chirho).into(),
                }
            })
            .collect(),
        Err(e_chirho) => {
            warn!("Failed to load notes: {}", e_chirho);
            Vec::new()
        }
    }
}

// ============================================================================
// Clipboard Support
// ============================================================================

/// Copy text to the system clipboard (cross-platform)
fn copy_to_clipboard_chirho(text_chirho: &str) -> bool {
    #[cfg(target_os = "macos")]
    {
        use std::io::Write;
        if let Ok(mut child_chirho) = StdCommand::new("pbcopy")
            .stdin(std::process::Stdio::piped())
            .spawn()
        {
            if let Some(stdin_chirho) = child_chirho.stdin.as_mut() {
                if stdin_chirho.write_all(text_chirho.as_bytes()).is_ok() {
                    return child_chirho.wait().map(|s_chirho| s_chirho.success()).unwrap_or(false);
                }
            }
        }
        false
    }

    #[cfg(target_os = "linux")]
    {
        use std::io::Write;
        // Try xclip first, then xsel
        if let Ok(mut child_chirho) = std::process::Command::new("xclip")
            .args(["-selection", "clipboard"])
            .stdin(std::process::Stdio::piped())
            .spawn()
        {
            if let Some(stdin_chirho) = child_chirho.stdin.as_mut() {
                if stdin_chirho.write_all(text_chirho.as_bytes()).is_ok() {
                    return child_chirho.wait().map(|s_chirho| s_chirho.success()).unwrap_or(false);
                }
            }
        }
        false
    }

    #[cfg(target_os = "windows")]
    {
        use std::io::Write;
        if let Ok(mut child_chirho) = std::process::Command::new("cmd")
            .args(["/C", "clip"])
            .stdin(std::process::Stdio::piped())
            .spawn()
        {
            if let Some(stdin_chirho) = child_chirho.stdin.as_mut() {
                if stdin_chirho.write_all(text_chirho.as_bytes()).is_ok() {
                    return child_chirho.wait().map(|s_chirho| s_chirho.success()).unwrap_or(false);
                }
            }
        }
        false
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        let _ = text_chirho;
        false
    }
}

// ============================================================================
// Reference Parsing
// ============================================================================

/// Parse a verse reference string like "John 3:16" or "Gen 1:1"
/// Returns (book, chapter, verse) or None if parsing fails
fn parse_reference_chirho(reference_chirho: &str) -> Option<(String, i32, i32)> {
    let reference_chirho = reference_chirho.trim();

    // Map common abbreviations to full book names
    let abbreviations_chirho: &[(&str, &str)] = &[
        ("genesis", "Genesis"), ("gen", "Genesis"),
        ("exodus", "Exodus"), ("exo", "Exodus"), ("ex", "Exodus"),
        ("leviticus", "Leviticus"), ("lev", "Leviticus"),
        ("numbers", "Numbers"), ("num", "Numbers"),
        ("deuteronomy", "Deuteronomy"), ("deut", "Deuteronomy"),
        ("joshua", "Joshua"), ("josh", "Joshua"),
        ("judges", "Judges"), ("judg", "Judges"), ("jdg", "Judges"),
        ("ruth", "Ruth"),
        ("1 samuel", "1 Samuel"), ("1samuel", "1 Samuel"), ("1sam", "1 Samuel"), ("1sa", "1 Samuel"),
        ("2 samuel", "2 Samuel"), ("2samuel", "2 Samuel"), ("2sam", "2 Samuel"), ("2sa", "2 Samuel"),
        ("1 kings", "1 Kings"), ("1kings", "1 Kings"), ("1ki", "1 Kings"), ("1kgs", "1 Kings"),
        ("2 kings", "2 Kings"), ("2kings", "2 Kings"), ("2ki", "2 Kings"), ("2kgs", "2 Kings"),
        ("1 chronicles", "1 Chronicles"), ("1chronicles", "1 Chronicles"), ("1chr", "1 Chronicles"), ("1ch", "1 Chronicles"),
        ("2 chronicles", "2 Chronicles"), ("2chronicles", "2 Chronicles"), ("2chr", "2 Chronicles"), ("2ch", "2 Chronicles"),
        ("ezra", "Ezra"), ("ezr", "Ezra"),
        ("nehemiah", "Nehemiah"), ("neh", "Nehemiah"),
        ("esther", "Esther"), ("esth", "Esther"), ("est", "Esther"),
        ("job", "Job"),
        ("psalms", "Psalms"), ("psalm", "Psalms"), ("psa", "Psalms"), ("ps", "Psalms"),
        ("proverbs", "Proverbs"), ("prov", "Proverbs"), ("pro", "Proverbs"),
        ("ecclesiastes", "Ecclesiastes"), ("eccl", "Ecclesiastes"), ("ecc", "Ecclesiastes"),
        ("song of solomon", "Song of Solomon"), ("song", "Song of Solomon"), ("sos", "Song of Solomon"),
        ("isaiah", "Isaiah"), ("isa", "Isaiah"),
        ("jeremiah", "Jeremiah"), ("jer", "Jeremiah"),
        ("lamentations", "Lamentations"), ("lam", "Lamentations"),
        ("ezekiel", "Ezekiel"), ("ezek", "Ezekiel"), ("eze", "Ezekiel"),
        ("daniel", "Daniel"), ("dan", "Daniel"),
        ("hosea", "Hosea"), ("hos", "Hosea"),
        ("joel", "Joel"),
        ("amos", "Amos"),
        ("obadiah", "Obadiah"), ("obad", "Obadiah"), ("oba", "Obadiah"),
        ("jonah", "Jonah"), ("jon", "Jonah"),
        ("micah", "Micah"), ("mic", "Micah"),
        ("nahum", "Nahum"), ("nah", "Nahum"),
        ("habakkuk", "Habakkuk"), ("hab", "Habakkuk"),
        ("zephaniah", "Zephaniah"), ("zeph", "Zephaniah"), ("zep", "Zephaniah"),
        ("haggai", "Haggai"), ("hag", "Haggai"),
        ("zechariah", "Zechariah"), ("zech", "Zechariah"), ("zec", "Zechariah"),
        ("malachi", "Malachi"), ("mal", "Malachi"),
        ("matthew", "Matthew"), ("matt", "Matthew"), ("mat", "Matthew"),
        ("mark", "Mark"), ("mk", "Mark"), ("mar", "Mark"),
        ("luke", "Luke"), ("lk", "Luke"), ("luk", "Luke"),
        ("john", "John"), ("jn", "John"), ("joh", "John"),
        ("acts", "Acts"),
        ("romans", "Romans"), ("rom", "Romans"),
        ("1 corinthians", "1 Corinthians"), ("1corinthians", "1 Corinthians"), ("1cor", "1 Corinthians"), ("1co", "1 Corinthians"),
        ("2 corinthians", "2 Corinthians"), ("2corinthians", "2 Corinthians"), ("2cor", "2 Corinthians"), ("2co", "2 Corinthians"),
        ("galatians", "Galatians"), ("gal", "Galatians"),
        ("ephesians", "Ephesians"), ("eph", "Ephesians"),
        ("philippians", "Philippians"), ("phil", "Philippians"), ("php", "Philippians"),
        ("colossians", "Colossians"), ("col", "Colossians"),
        ("1 thessalonians", "1 Thessalonians"), ("1thessalonians", "1 Thessalonians"), ("1thess", "1 Thessalonians"), ("1th", "1 Thessalonians"),
        ("2 thessalonians", "2 Thessalonians"), ("2thessalonians", "2 Thessalonians"), ("2thess", "2 Thessalonians"), ("2th", "2 Thessalonians"),
        ("1 timothy", "1 Timothy"), ("1timothy", "1 Timothy"), ("1tim", "1 Timothy"), ("1ti", "1 Timothy"),
        ("2 timothy", "2 Timothy"), ("2timothy", "2 Timothy"), ("2tim", "2 Timothy"), ("2ti", "2 Timothy"),
        ("titus", "Titus"), ("tit", "Titus"),
        ("philemon", "Philemon"), ("phm", "Philemon"), ("phlm", "Philemon"),
        ("hebrews", "Hebrews"), ("heb", "Hebrews"),
        ("james", "James"), ("jas", "James"), ("jam", "James"),
        ("1 peter", "1 Peter"), ("1peter", "1 Peter"), ("1pet", "1 Peter"), ("1pe", "1 Peter"),
        ("2 peter", "2 Peter"), ("2peter", "2 Peter"), ("2pet", "2 Peter"), ("2pe", "2 Peter"),
        ("1 john", "1 John"), ("1john", "1 John"), ("1jn", "1 John"), ("1jo", "1 John"),
        ("2 john", "2 John"), ("2john", "2 John"), ("2jn", "2 John"), ("2jo", "2 John"),
        ("3 john", "3 John"), ("3john", "3 John"), ("3jn", "3 John"), ("3jo", "3 John"),
        ("jude", "Jude"),
        ("revelation", "Revelation"), ("rev", "Revelation"),
    ];

    // Normalize reference (handle both "1 Cor 13:4" and "1Cor 13:4")
    let reference_lower_chirho = reference_chirho.to_lowercase();

    // Find the chapter:verse part by looking for the last digit-colon-digit or standalone digit pattern
    // We need to find where the book name ends and chapter:verse begins
    // This is tricky because books like "1 Corinthians" have numbers

    // Try to match each abbreviation against the start of the reference
    for (abbr_chirho, book_name_chirho) in abbreviations_chirho {
        let abbr_len_chirho = abbr_chirho.len();
        if reference_lower_chirho.len() > abbr_len_chirho
            && reference_lower_chirho.starts_with(abbr_chirho)
        {
            // Check that next char after abbreviation is space, digit, or nothing
            let next_char_chirho = reference_lower_chirho.chars().nth(abbr_len_chirho);
            if matches!(next_char_chirho, Some(' ') | Some(':') | None) ||
               next_char_chirho.map(|c_chirho| c_chirho.is_ascii_digit()).unwrap_or(false) {

                let remainder_chirho = reference_chirho[abbr_len_chirho..].trim();

                // Parse chapter:verse from remainder
                let cv_str_chirho = remainder_chirho.trim_start();
                if cv_str_chirho.is_empty() {
                    continue;
                }

                let cv_parts_chirho: Vec<&str> = cv_str_chirho.split(':').collect();
                if let Ok(chapter_chirho) = cv_parts_chirho.first()
                    .unwrap_or(&"")
                    .trim()
                    .parse::<i32>()
                {
                    // Handle verse ranges like "1-10" by extracting the start verse
                    let verse_str_chirho = cv_parts_chirho.get(1).map(|s_chirho| s_chirho.trim()).unwrap_or("1");
                    let verse_start_chirho = verse_str_chirho
                        .split('-')
                        .next()
                        .and_then(|v_chirho| v_chirho.trim().parse::<i32>().ok())
                        .unwrap_or(1);

                    return Some((book_name_chirho.to_string(), chapter_chirho, verse_start_chirho));
                }
            }
        }
    }

    None
}

// ============================================================================
// Quick Navigation Search
// ============================================================================

/// Search for book names and recent locations for quick navigation
fn quick_nav_search_chirho(query_chirho: &str, conn_chirho: &Connection) -> Vec<HistoryEntryChirho> {
    let query_lower_chirho = query_chirho.to_lowercase();
    let mut results_chirho: Vec<HistoryEntryChirho> = Vec::new();

    // If query is empty, show recent history
    if query_chirho.is_empty() {
        if let Ok(history_chirho) = database_chirho::get_recent_history_chirho(conn_chirho, 10) {
            for entry_chirho in history_chirho {
                results_chirho.push(HistoryEntryChirho {
                    book_chirho: entry_chirho.book_chirho.into(),
                    chapter_chirho: entry_chirho.chapter_chirho,
                    timestamp_chirho: format_timestamp_chirho(&entry_chirho.timestamp_chirho).into(),
                });
            }
        }
        return results_chirho;
    }

    // Search for matching books with fuzzy matching
    for (book_name_chirho, chapter_count_chirho) in BIBLE_BOOKS_CHIRHO.iter() {
        let book_lower_chirho = book_name_chirho.to_lowercase();

        // Check if query matches start of book name or is contained within
        if book_lower_chirho.starts_with(&query_lower_chirho) ||
           book_lower_chirho.contains(&query_lower_chirho) ||
           // Check common abbreviations
           query_lower_chirho.len() >= 2 && book_lower_chirho.starts_with(&query_lower_chirho[..query_lower_chirho.len().min(3)]) {
            // Add first chapter of matching book
            results_chirho.push(HistoryEntryChirho {
                book_chirho: (*book_name_chirho).into(),
                chapter_chirho: 1,
                timestamp_chirho: "".into(),
            });

            // Also show all chapters if book name matches exactly
            if book_lower_chirho == query_lower_chirho && results_chirho.len() < 10 {
                for ch_chirho in 2..=(*chapter_count_chirho).min(10) {
                    results_chirho.push(HistoryEntryChirho {
                        book_chirho: (*book_name_chirho).into(),
                        chapter_chirho: ch_chirho,
                        timestamp_chirho: "".into(),
                    });
                }
            }

            if results_chirho.len() >= 15 {
                break;
            }
        }
    }

    // If query looks like a verse reference (contains numbers), try to parse it
    if query_chirho.chars().any(|c_chirho| c_chirho.is_ascii_digit()) {
        if let Some((book_chirho, chapter_chirho, _verse_chirho)) = parse_reference_chirho(query_chirho) {
            // Add parsed reference at the top
            let parsed_entry_chirho = HistoryEntryChirho {
                book_chirho: book_chirho.into(),
                chapter_chirho,
                timestamp_chirho: "".into(),
            };

            // Don't add if already exists
            if !results_chirho.iter().any(|e_chirho|
                e_chirho.book_chirho == parsed_entry_chirho.book_chirho &&
                e_chirho.chapter_chirho == parsed_entry_chirho.chapter_chirho
            ) {
                results_chirho.insert(0, parsed_entry_chirho);
            }
        }
    }

    // Limit results
    results_chirho.truncate(15);
    results_chirho
}

// ============================================================================
// Export Functions for Data Portability
// ============================================================================

/// Get the export directory path (creates if needed)
fn get_export_directory_chirho() -> Result<PathBuf> {
    if let Some(proj_dirs_chirho) = ProjectDirs::from(
        APP_QUALIFIER_CHIRHO,
        APP_ORGANIZATION_CHIRHO,
        APP_NAME_CHIRHO,
    ) {
        let export_dir_chirho = proj_dirs_chirho.data_dir().join("exports");
        std::fs::create_dir_all(&export_dir_chirho)?;
        Ok(export_dir_chirho)
    } else {
        Err(anyhow::anyhow!("Could not determine export directory"))
    }
}

/// Export highlights to JSON file
fn export_highlights_to_json_chirho(conn_chirho: &Connection) -> Result<PathBuf> {
    use std::io::Write;

    let highlights_chirho = database_chirho::get_all_highlights_chirho(conn_chirho)?;
    let export_dir_chirho = get_export_directory_chirho()?;

    let timestamp_chirho = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let filename_chirho = format!("highlights_{}.json", timestamp_chirho);
    let path_chirho = export_dir_chirho.join(&filename_chirho);

    let json_data_chirho: Vec<serde_json::Value> = highlights_chirho
        .iter()
        .map(|hl_chirho| {
            serde_json::json!({
                "reference": format!("{} {}:{}", hl_chirho.book_chirho, hl_chirho.chapter_chirho, hl_chirho.verse_chirho),
                "book": hl_chirho.book_chirho,
                "chapter": hl_chirho.chapter_chirho,
                "verse": hl_chirho.verse_chirho,
                "color": hl_chirho.color_chirho,
                "module": hl_chirho.module_chirho,
            })
        })
        .collect();

    let json_str_chirho = serde_json::to_string_pretty(&json_data_chirho)?;
    let mut file_chirho = std::fs::File::create(&path_chirho)?;
    file_chirho.write_all(json_str_chirho.as_bytes())?;

    info!("Exported {} highlights to {:?}", highlights_chirho.len(), path_chirho);
    Ok(path_chirho)
}

/// Export bookmarks to JSON file
fn export_bookmarks_to_json_chirho(conn_chirho: &Connection) -> Result<PathBuf> {
    use std::io::Write;

    let bookmarks_chirho = database_chirho::get_all_bookmarks_chirho(conn_chirho)?;
    let export_dir_chirho = get_export_directory_chirho()?;

    let timestamp_chirho = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let filename_chirho = format!("bookmarks_{}.json", timestamp_chirho);
    let path_chirho = export_dir_chirho.join(&filename_chirho);

    let json_data_chirho: Vec<serde_json::Value> = bookmarks_chirho
        .iter()
        .map(|bm_chirho| {
            serde_json::json!({
                "reference": format!("{} {}:{}", bm_chirho.book_chirho, bm_chirho.chapter_chirho, bm_chirho.verse_chirho),
                "book": bm_chirho.book_chirho,
                "chapter": bm_chirho.chapter_chirho,
                "verse": bm_chirho.verse_chirho,
                "label": bm_chirho.label_chirho,
                "module": bm_chirho.module_chirho,
            })
        })
        .collect();

    let json_str_chirho = serde_json::to_string_pretty(&json_data_chirho)?;
    let mut file_chirho = std::fs::File::create(&path_chirho)?;
    file_chirho.write_all(json_str_chirho.as_bytes())?;

    info!("Exported {} bookmarks to {:?}", bookmarks_chirho.len(), path_chirho);
    Ok(path_chirho)
}

/// Export notes to JSON file
fn export_notes_to_json_chirho(conn_chirho: &Connection) -> Result<PathBuf> {
    use std::io::Write;

    let notes_chirho = database_chirho::get_all_notes_chirho(conn_chirho)?;
    let export_dir_chirho = get_export_directory_chirho()?;

    let timestamp_chirho = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let filename_chirho = format!("notes_{}.json", timestamp_chirho);
    let path_chirho = export_dir_chirho.join(&filename_chirho);

    let json_data_chirho: Vec<serde_json::Value> = notes_chirho
        .iter()
        .map(|note_chirho| {
            serde_json::json!({
                "reference": format!("{} {}:{}", note_chirho.book_chirho, note_chirho.chapter_chirho, note_chirho.verse_chirho),
                "book": note_chirho.book_chirho,
                "chapter": note_chirho.chapter_chirho,
                "verse": note_chirho.verse_chirho,
                "content": note_chirho.content_chirho,
                "module": note_chirho.module_chirho,
                "created_at": note_chirho.created_at_chirho,
                "updated_at": note_chirho.updated_at_chirho,
            })
        })
        .collect();

    let json_str_chirho = serde_json::to_string_pretty(&json_data_chirho)?;
    let mut file_chirho = std::fs::File::create(&path_chirho)?;
    file_chirho.write_all(json_str_chirho.as_bytes())?;

    info!("Exported {} notes to {:?}", notes_chirho.len(), path_chirho);
    Ok(path_chirho)
}

/// Export all user data to a single JSON file
fn export_all_data_to_json_chirho(conn_chirho: &Connection) -> Result<PathBuf> {
    use std::io::Write;

    let highlights_chirho = database_chirho::get_all_highlights_chirho(conn_chirho)?;
    let bookmarks_chirho = database_chirho::get_all_bookmarks_chirho(conn_chirho)?;
    let notes_chirho = database_chirho::get_all_notes_chirho(conn_chirho)?;

    let export_dir_chirho = get_export_directory_chirho()?;
    let timestamp_chirho = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let filename_chirho = format!("codex_lux_backup_{}.json", timestamp_chirho);
    let path_chirho = export_dir_chirho.join(&filename_chirho);

    let json_data_chirho = serde_json::json!({
        "export_info": {
            "app": "Codex Lux Chirho",
            "version": env!("CARGO_PKG_VERSION"),
            "exported_at": chrono::Local::now().to_rfc3339(),
        },
        "highlights": highlights_chirho.iter().map(|hl_chirho| {
            serde_json::json!({
                "reference": format!("{} {}:{}", hl_chirho.book_chirho, hl_chirho.chapter_chirho, hl_chirho.verse_chirho),
                "book": hl_chirho.book_chirho,
                "chapter": hl_chirho.chapter_chirho,
                "verse": hl_chirho.verse_chirho,
                "color": hl_chirho.color_chirho,
                "module": hl_chirho.module_chirho,
            })
        }).collect::<Vec<_>>(),
        "bookmarks": bookmarks_chirho.iter().map(|bm_chirho| {
            serde_json::json!({
                "reference": format!("{} {}:{}", bm_chirho.book_chirho, bm_chirho.chapter_chirho, bm_chirho.verse_chirho),
                "book": bm_chirho.book_chirho,
                "chapter": bm_chirho.chapter_chirho,
                "verse": bm_chirho.verse_chirho,
                "label": bm_chirho.label_chirho,
                "module": bm_chirho.module_chirho,
            })
        }).collect::<Vec<_>>(),
        "notes": notes_chirho.iter().map(|note_chirho| {
            serde_json::json!({
                "reference": format!("{} {}:{}", note_chirho.book_chirho, note_chirho.chapter_chirho, note_chirho.verse_chirho),
                "book": note_chirho.book_chirho,
                "chapter": note_chirho.chapter_chirho,
                "verse": note_chirho.verse_chirho,
                "content": note_chirho.content_chirho,
                "module": note_chirho.module_chirho,
                "created_at": note_chirho.created_at_chirho,
                "updated_at": note_chirho.updated_at_chirho,
            })
        }).collect::<Vec<_>>(),
    });

    let json_str_chirho = serde_json::to_string_pretty(&json_data_chirho)?;
    let mut file_chirho = std::fs::File::create(&path_chirho)?;
    file_chirho.write_all(json_str_chirho.as_bytes())?;

    info!(
        "Exported all data ({} highlights, {} bookmarks, {} notes) to {:?}",
        highlights_chirho.len(),
        bookmarks_chirho.len(),
        notes_chirho.len(),
        path_chirho
    );
    Ok(path_chirho)
}

/// Import user data from a JSON backup file
fn import_data_from_json_chirho(conn_chirho: &Connection, file_path_chirho: &str) -> Result<(usize, usize, usize)> {
    use std::io::Read;

    // Read the JSON file
    let mut file_chirho = std::fs::File::open(file_path_chirho)?;
    let mut json_str_chirho = String::new();
    file_chirho.read_to_string(&mut json_str_chirho)?;

    let json_data_chirho: serde_json::Value = serde_json::from_str(&json_str_chirho)?;

    let mut highlights_imported_chirho = 0;
    let mut bookmarks_imported_chirho = 0;
    let mut notes_imported_chirho = 0;

    // Import highlights
    if let Some(highlights_chirho) = json_data_chirho.get("highlights").and_then(|v_chirho| v_chirho.as_array()) {
        for hl_chirho in highlights_chirho {
            if let (Some(book_chirho), Some(chapter_chirho), Some(verse_chirho)) = (
                hl_chirho.get("book").and_then(|v_chirho| v_chirho.as_str()),
                hl_chirho.get("chapter").and_then(|v_chirho| v_chirho.as_i64()),
                hl_chirho.get("verse").and_then(|v_chirho| v_chirho.as_i64()),
            ) {
                let color_chirho = hl_chirho.get("color").and_then(|v_chirho| v_chirho.as_str()).unwrap_or("yellow");
                let module_chirho = hl_chirho.get("module").and_then(|v_chirho| v_chirho.as_str()).unwrap_or("KJV");

                // Use upsert to avoid duplicates
                if database_chirho::add_highlight_with_color_chirho(
                    conn_chirho,
                    module_chirho,
                    book_chirho,
                    chapter_chirho as i32,
                    verse_chirho as i32,
                    color_chirho,
                ).is_ok() {
                    highlights_imported_chirho += 1;
                }
            }
        }
    }

    // Import bookmarks
    if let Some(bookmarks_chirho) = json_data_chirho.get("bookmarks").and_then(|v_chirho| v_chirho.as_array()) {
        for bm_chirho in bookmarks_chirho {
            if let (Some(book_chirho), Some(chapter_chirho), Some(verse_chirho)) = (
                bm_chirho.get("book").and_then(|v_chirho| v_chirho.as_str()),
                bm_chirho.get("chapter").and_then(|v_chirho| v_chirho.as_i64()),
                bm_chirho.get("verse").and_then(|v_chirho| v_chirho.as_i64()),
            ) {
                let label_chirho = bm_chirho.get("label").and_then(|v_chirho| v_chirho.as_str());
                let module_chirho = bm_chirho.get("module").and_then(|v_chirho| v_chirho.as_str()).unwrap_or("KJV");

                // Add bookmark (may fail if duplicate exists)
                if database_chirho::add_bookmark_chirho(
                    conn_chirho,
                    module_chirho,
                    book_chirho,
                    chapter_chirho as i32,
                    verse_chirho as i32,
                    label_chirho,
                ).is_ok() {
                    bookmarks_imported_chirho += 1;
                }
            }
        }
    }

    // Import notes
    if let Some(notes_chirho) = json_data_chirho.get("notes").and_then(|v_chirho| v_chirho.as_array()) {
        for note_chirho in notes_chirho {
            if let (Some(book_chirho), Some(chapter_chirho), Some(verse_chirho), Some(content_chirho)) = (
                note_chirho.get("book").and_then(|v_chirho| v_chirho.as_str()),
                note_chirho.get("chapter").and_then(|v_chirho| v_chirho.as_i64()),
                note_chirho.get("verse").and_then(|v_chirho| v_chirho.as_i64()),
                note_chirho.get("content").and_then(|v_chirho| v_chirho.as_str()),
            ) {
                let module_chirho = note_chirho.get("module").and_then(|v_chirho| v_chirho.as_str()).unwrap_or("KJV");

                // Upsert note
                if database_chirho::save_note_chirho(
                    conn_chirho,
                    module_chirho,
                    book_chirho,
                    chapter_chirho as i32,
                    verse_chirho as i32,
                    content_chirho,
                ).is_ok() {
                    notes_imported_chirho += 1;
                }
            }
        }
    }

    info!(
        "Imported data: {} highlights, {} bookmarks, {} notes from {:?}",
        highlights_imported_chirho,
        bookmarks_imported_chirho,
        notes_imported_chirho,
        file_path_chirho
    );

    Ok((highlights_imported_chirho, bookmarks_imported_chirho, notes_imported_chirho))
}

/// Get list of available backup files in the export directory
fn list_backup_files_chirho() -> Result<Vec<String>> {
    let export_dir_chirho = get_export_directory_chirho()?;

    let mut backups_chirho: Vec<String> = Vec::new();

    if let Ok(entries_chirho) = std::fs::read_dir(&export_dir_chirho) {
        for entry_chirho in entries_chirho.flatten() {
            let path_chirho = entry_chirho.path();
            if path_chirho.extension().is_some_and(|ext_chirho| ext_chirho == "json") {
                if let Some(filename_chirho) = path_chirho.file_name() {
                    if let Some(name_chirho) = filename_chirho.to_str() {
                        if name_chirho.starts_with("codex_lux_backup_") {
                            backups_chirho.push(name_chirho.to_string());
                        }
                    }
                }
            }
        }
    }

    // Sort by name (which includes timestamp, so newest first when reversed)
    backups_chirho.sort();
    backups_chirho.reverse();

    Ok(backups_chirho)
}

/// Get sample interlinear word data for a verse (CLX-051)
/// In production, this would query an interlinear module via rsword_chirho
fn get_sample_interlinear_chirho(verse_ref_chirho: &str) -> Vec<InterlinearWordChirho> {
    match verse_ref_chirho {
        v if v.contains("Genesis 1:1") || v.contains("Genesis 1") => vec![
            InterlinearWordChirho {
                original_chirho: "בְּרֵאשִׁ֖ית".into(),
                transliteration_chirho: "bəreʾšiṯ".into(),
                morphology_chirho: "Prep-b | N-fs".into(),
                strongs_chirho: "H7225".into(),
                gloss_chirho: "In [the] beginning".into(),
                part_of_speech_chirho: "Noun".into(),
                is_hebrew_chirho: true,
            },
            InterlinearWordChirho {
                original_chirho: "בָּרָ֣א".into(),
                transliteration_chirho: "bārāʾ".into(),
                morphology_chirho: "V-Qal-Perf-3ms".into(),
                strongs_chirho: "H1254".into(),
                gloss_chirho: "created".into(),
                part_of_speech_chirho: "Verb".into(),
                is_hebrew_chirho: true,
            },
            InterlinearWordChirho {
                original_chirho: "אֱלֹהִ֑ים".into(),
                transliteration_chirho: "ʾĕlōhîm".into(),
                morphology_chirho: "N-mp".into(),
                strongs_chirho: "H430".into(),
                gloss_chirho: "God".into(),
                part_of_speech_chirho: "Noun".into(),
                is_hebrew_chirho: true,
            },
            InterlinearWordChirho {
                original_chirho: "אֵ֥ת".into(),
                transliteration_chirho: "ʾēṯ".into(),
                morphology_chirho: "DirObjM".into(),
                strongs_chirho: "H853".into(),
                gloss_chirho: "[direct object]".into(),
                part_of_speech_chirho: "Particle".into(),
                is_hebrew_chirho: true,
            },
            InterlinearWordChirho {
                original_chirho: "הַשָּׁמַ֖יִם".into(),
                transliteration_chirho: "haššāmayim".into(),
                morphology_chirho: "Art | N-mp".into(),
                strongs_chirho: "H8064".into(),
                gloss_chirho: "the heavens".into(),
                part_of_speech_chirho: "Noun".into(),
                is_hebrew_chirho: true,
            },
            InterlinearWordChirho {
                original_chirho: "וְאֵ֥ת".into(),
                transliteration_chirho: "wəʾēṯ".into(),
                morphology_chirho: "Conj-w | DirObjM".into(),
                strongs_chirho: "H853".into(),
                gloss_chirho: "and [direct object]".into(),
                part_of_speech_chirho: "Conjunction".into(),
                is_hebrew_chirho: true,
            },
            InterlinearWordChirho {
                original_chirho: "הָאָֽרֶץ".into(),
                transliteration_chirho: "hāʾāreṣ".into(),
                morphology_chirho: "Art | N-fs".into(),
                strongs_chirho: "H776".into(),
                gloss_chirho: "the earth".into(),
                part_of_speech_chirho: "Noun".into(),
                is_hebrew_chirho: true,
            },
        ],
        v if v.contains("John 3:16") || v.contains("John 3") => vec![
            InterlinearWordChirho {
                original_chirho: "Οὕτως".into(),
                transliteration_chirho: "houtōs".into(),
                morphology_chirho: "Adv".into(),
                strongs_chirho: "G3779".into(),
                gloss_chirho: "For so".into(),
                part_of_speech_chirho: "Adverb".into(),
                is_hebrew_chirho: false,
            },
            InterlinearWordChirho {
                original_chirho: "γὰρ".into(),
                transliteration_chirho: "gar".into(),
                morphology_chirho: "Conj".into(),
                strongs_chirho: "G1063".into(),
                gloss_chirho: "for".into(),
                part_of_speech_chirho: "Conjunction".into(),
                is_hebrew_chirho: false,
            },
            InterlinearWordChirho {
                original_chirho: "ἠγάπησεν".into(),
                transliteration_chirho: "ēgapēsen".into(),
                morphology_chirho: "V-AAI-3S".into(),
                strongs_chirho: "G25".into(),
                gloss_chirho: "loved".into(),
                part_of_speech_chirho: "Verb".into(),
                is_hebrew_chirho: false,
            },
            InterlinearWordChirho {
                original_chirho: "ὁ θεὸς".into(),
                transliteration_chirho: "ho theos".into(),
                morphology_chirho: "Art | N-NMS".into(),
                strongs_chirho: "G2316".into(),
                gloss_chirho: "God".into(),
                part_of_speech_chirho: "Noun".into(),
                is_hebrew_chirho: false,
            },
            InterlinearWordChirho {
                original_chirho: "τὸν κόσμον".into(),
                transliteration_chirho: "ton kosmon".into(),
                morphology_chirho: "Art | N-AMS".into(),
                strongs_chirho: "G2889".into(),
                gloss_chirho: "the world".into(),
                part_of_speech_chirho: "Noun".into(),
                is_hebrew_chirho: false,
            },
            InterlinearWordChirho {
                original_chirho: "ὥστε".into(),
                transliteration_chirho: "hōste".into(),
                morphology_chirho: "Conj".into(),
                strongs_chirho: "G5620".into(),
                gloss_chirho: "that".into(),
                part_of_speech_chirho: "Conjunction".into(),
                is_hebrew_chirho: false,
            },
            InterlinearWordChirho {
                original_chirho: "τὸν υἱὸν".into(),
                transliteration_chirho: "ton huion".into(),
                morphology_chirho: "Art | N-AMS".into(),
                strongs_chirho: "G5207".into(),
                gloss_chirho: "the Son".into(),
                part_of_speech_chirho: "Noun".into(),
                is_hebrew_chirho: false,
            },
            InterlinearWordChirho {
                original_chirho: "τὸν μονογενῆ".into(),
                transliteration_chirho: "ton monogenē".into(),
                morphology_chirho: "Art | Adj-AMS".into(),
                strongs_chirho: "G3439".into(),
                gloss_chirho: "the only begotten".into(),
                part_of_speech_chirho: "Adjective".into(),
                is_hebrew_chirho: false,
            },
            InterlinearWordChirho {
                original_chirho: "ἔδωκεν".into(),
                transliteration_chirho: "edōken".into(),
                morphology_chirho: "V-AAI-3S".into(),
                strongs_chirho: "G1325".into(),
                gloss_chirho: "He gave".into(),
                part_of_speech_chirho: "Verb".into(),
                is_hebrew_chirho: false,
            },
        ],
        _ => vec![
            InterlinearWordChirho {
                original_chirho: "[Sample]".into(),
                transliteration_chirho: "sample".into(),
                morphology_chirho: "N/A".into(),
                strongs_chirho: "".into(),
                gloss_chirho: "Interlinear data not available".into(),
                part_of_speech_chirho: "Info".into(),
                is_hebrew_chirho: false,
            },
        ],
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_bible_book_count_chirho() {
        assert_eq!(BIBLE_BOOKS_CHIRHO.len(), BIBLE_BOOK_COUNT_CHIRHO);
    }

    #[test]
    fn test_old_testament_count_chirho() {
        // Old Testament ends at Malachi (index 38, 0-indexed)
        assert_eq!(OLD_TESTAMENT_BOOK_COUNT_CHIRHO, 39);
    }

    #[test]
    fn test_new_testament_count_chirho() {
        assert_eq!(NEW_TESTAMENT_BOOK_COUNT_CHIRHO, 27);
        assert_eq!(
            OLD_TESTAMENT_BOOK_COUNT_CHIRHO + NEW_TESTAMENT_BOOK_COUNT_CHIRHO,
            BIBLE_BOOK_COUNT_CHIRHO
        );
    }

    #[test]
    fn test_genesis_chapters_chirho() {
        let genesis_chirho = BIBLE_BOOKS_CHIRHO.iter().find(|(name, _)| *name == "Genesis");
        assert!(genesis_chirho.is_some());
        assert_eq!(genesis_chirho.unwrap().1, 50);
    }

    #[test]
    fn test_psalms_chapters_chirho() {
        let psalms_chirho = BIBLE_BOOKS_CHIRHO.iter().find(|(name, _)| *name == "Psalms");
        assert!(psalms_chirho.is_some());
        assert_eq!(psalms_chirho.unwrap().1, 150);
    }

    #[test]
    fn test_revelation_chapters_chirho() {
        let revelation_chirho = BIBLE_BOOKS_CHIRHO.iter().find(|(name, _)| *name == "Revelation");
        assert!(revelation_chirho.is_some());
        assert_eq!(revelation_chirho.unwrap().1, 22);
    }

    #[test]
    fn test_sample_verses_genesis_chirho() {
        let verses_chirho = get_sample_verses_chirho("Genesis", 1);
        assert!(!verses_chirho.is_empty());
        assert_eq!(verses_chirho[0].0, "1");
        assert!(verses_chirho[0].1.contains("In the beginning"));
    }

    #[test]
    fn test_sample_verses_john_chirho() {
        let verses_chirho = get_sample_verses_chirho("John", 3);
        assert!(!verses_chirho.is_empty());
        // John 3:16 should be present
        let has_john_316_chirho = verses_chirho.iter().any(|(_, text)| text.contains("For God so loved the world"));
        assert!(has_john_316_chirho);
    }

    #[test]
    fn test_sample_verses_psalms_chirho() {
        let verses_chirho = get_sample_verses_chirho("Psalms", 23);
        assert!(!verses_chirho.is_empty());
        // Should contain Hebrew text
        let has_hebrew_chirho = verses_chirho.iter().any(|(_, text)| text.contains("יְהוָ֥ה"));
        assert!(has_hebrew_chirho);
    }

    #[test]
    fn test_sample_verses_fallback_chirho() {
        let verses_chirho = get_sample_verses_chirho("Exodus", 1);
        assert!(!verses_chirho.is_empty());
        assert!(verses_chirho[0].1.contains("Sample verse"));
    }

    #[test]
    fn test_strongs_definition_chirho() {
        // Test Hebrew Strong's lookup
        let hebrew_entry_chirho = get_strongs_definition_chirho("H430");
        assert_eq!(hebrew_entry_chirho.number_chirho.as_str(), "H430");
        assert!(hebrew_entry_chirho.is_hebrew_chirho);
        assert!(hebrew_entry_chirho.lemma_chirho.to_string().contains("אֱלֹהִים"));
        assert!(hebrew_entry_chirho.definition_chirho.to_string().contains("God"));
        assert!(hebrew_entry_chirho.occurrences_chirho > 0);

        // Test Greek Strong's lookup
        let greek_entry_chirho = get_strongs_definition_chirho("G26");
        assert_eq!(greek_entry_chirho.number_chirho.as_str(), "G26");
        assert!(!greek_entry_chirho.is_hebrew_chirho);
        assert!(greek_entry_chirho.lemma_chirho.to_string().contains("ἀγάπη"));
        assert!(greek_entry_chirho.definition_chirho.to_string().contains("Love"));

        // Test unknown Strong's number
        let unknown_entry_chirho = get_strongs_definition_chirho("H99999");
        assert!(unknown_entry_chirho.definition_chirho.to_string().contains("not found"));
    }

    #[test]
    fn test_morphology_parsing_chirho() {
        // Test Greek morphology parsing (Robinson codes)
        let (greek_parsed_chirho, is_hebrew_chirho) = parse_morphology_code_chirho("V-AAI-3S");
        assert!(!is_hebrew_chirho);
        assert!(greek_parsed_chirho.contains("Verb"));
        assert!(greek_parsed_chirho.contains("Aorist Active Indicative"));
        assert!(greek_parsed_chirho.contains("3rd Person Singular"));

        // Test another Greek code
        let (greek_noun_chirho, _) = parse_morphology_code_chirho("N-NSM");
        assert!(greek_noun_chirho.contains("Noun"));
        assert!(greek_noun_chirho.contains("Nominative Singular Masculine"));

        // Test Hebrew morphology parsing (no hyphens)
        let (hebrew_parsed_chirho, is_hebrew_chirho) = parse_morphology_code_chirho("Ncmsa");
        assert!(is_hebrew_chirho);
        assert!(hebrew_parsed_chirho.contains("Noun"));
        assert!(hebrew_parsed_chirho.contains("masculine"));
        assert!(hebrew_parsed_chirho.contains("singular"));
        assert!(hebrew_parsed_chirho.contains("absolute"));
    }

    #[test]
    fn test_cross_refs_chirho() {
        // Test known verse with cross-references
        let refs_chirho = get_sample_cross_refs_chirho("John 3:16");
        assert!(!refs_chirho.is_empty());
        assert!(refs_chirho.iter().any(|r_chirho| r_chirho.to_string().contains("Romans")));

        // Test Genesis 1:1
        let gen_refs_chirho = get_sample_cross_refs_chirho("Genesis 1:1");
        assert!(!gen_refs_chirho.is_empty());
        assert!(gen_refs_chirho.iter().any(|r_chirho| r_chirho.to_string().contains("John 1:1")));

        // Test unknown verse falls back to install message
        let unknown_refs_chirho = get_sample_cross_refs_chirho("Unknown 99:99");
        assert!(!unknown_refs_chirho.is_empty());
        assert!(unknown_refs_chirho[0].to_string().contains("Install"));
    }

    #[test]
    fn test_footnotes_chirho() {
        // Test known footnote
        let (content_chirho, type_chirho) = get_sample_footnote_chirho("Genesis 1:1", "a");
        assert!(content_chirho.contains("beginning"));
        assert_eq!(type_chirho, "alternative");

        // Test John 3:16 footnote
        let (jn_content_chirho, jn_type_chirho) = get_sample_footnote_chirho("John 3:16", "a");
        assert!(jn_content_chirho.contains("Son"));
        assert_eq!(jn_type_chirho, "textual");

        // Test unknown footnote has verse ref in content
        let (unknown_content_chirho, unknown_type_chirho) = get_sample_footnote_chirho("Unknown 1:1", "z");
        assert!(unknown_content_chirho.contains("z"));
        assert_eq!(unknown_type_chirho, "translator");
    }

    #[test]
    fn test_get_chapter_count_chirho() {
        assert_eq!(AppBackendChirho::get_chapter_count_chirho("Genesis"), Some(50));
        assert_eq!(AppBackendChirho::get_chapter_count_chirho("Psalms"), Some(150));
        assert_eq!(AppBackendChirho::get_chapter_count_chirho("InvalidBook"), None);
    }

    #[test]
    fn test_is_valid_book_chirho() {
        assert!(AppBackendChirho::is_valid_book_chirho("Genesis"));
        assert!(AppBackendChirho::is_valid_book_chirho("Revelation"));
        assert!(!AppBackendChirho::is_valid_book_chirho("InvalidBook"));
    }

    #[test]
    fn test_database_init_chirho() {
        use tempfile::tempdir;

        let temp_dir_chirho = tempdir().unwrap();
        let db_path_chirho = temp_dir_chirho.path().join("test.db");

        let conn_chirho = database_chirho::init_database_chirho(&db_path_chirho).unwrap();

        // Verify tables exist by querying them
        let tables_chirho: Vec<String> = conn_chirho
            .prepare("SELECT name FROM sqlite_master WHERE type='table'")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();

        assert!(tables_chirho.contains(&"highlights_chirho".to_string()));
        assert!(tables_chirho.contains(&"bookmarks_chirho".to_string()));
        assert!(tables_chirho.contains(&"notes_chirho".to_string()));
        assert!(tables_chirho.contains(&"settings_chirho".to_string()));
    }

    #[test]
    fn test_highlight_toggle_database_chirho() {
        use tempfile::tempdir;

        let temp_dir_chirho = tempdir().unwrap();
        let db_path_chirho = temp_dir_chirho.path().join("test_highlight.db");

        let conn_chirho = database_chirho::init_database_chirho(&db_path_chirho).unwrap();

        // Toggle on
        let result_chirho = database_chirho::toggle_highlight_chirho(&conn_chirho, "KJV", "Genesis", 1, 1).unwrap();
        assert!(result_chirho); // Should be highlighted

        // Verify it's in the set
        let highlights_chirho = database_chirho::get_highlights_chirho(&conn_chirho, "KJV", "Genesis", 1).unwrap();
        assert!(highlights_chirho.contains(&1));

        // Toggle off
        let result_chirho = database_chirho::toggle_highlight_chirho(&conn_chirho, "KJV", "Genesis", 1, 1).unwrap();
        assert!(!result_chirho); // Should not be highlighted

        // Verify it's removed
        let highlights_chirho = database_chirho::get_highlights_chirho(&conn_chirho, "KJV", "Genesis", 1).unwrap();
        assert!(!highlights_chirho.contains(&1));
    }

    #[test]
    fn test_settings_chirho() {
        use tempfile::tempdir;

        let temp_dir_chirho = tempdir().unwrap();
        let db_path_chirho = temp_dir_chirho.path().join("test_settings.db");

        let conn_chirho = database_chirho::init_database_chirho(&db_path_chirho).unwrap();

        // Set a setting
        database_chirho::set_setting_chirho(&conn_chirho, "test_key", "test_value").unwrap();

        // Get the setting
        let value_chirho = database_chirho::get_setting_chirho(&conn_chirho, "test_key");
        assert_eq!(value_chirho, Some("test_value".to_string()));

        // Get non-existent setting
        let none_chirho = database_chirho::get_setting_chirho(&conn_chirho, "nonexistent");
        assert!(none_chirho.is_none());
    }

    #[test]
    fn test_bible_engine_chirho() {
        let engine_chirho = bible_engine_chirho::BibleEngineChirho::new_chirho();

        // Should have at least the default modules
        let modules_chirho = engine_chirho.get_module_names_chirho();
        assert!(!modules_chirho.is_empty());

        // Should be able to get sample verses
        let verses_chirho = engine_chirho.get_chapter_verses_chirho("Genesis", 1);
        assert!(!verses_chirho.is_empty());
    }

    #[test]
    fn test_search_chirho() {
        let engine_chirho = bible_engine_chirho::BibleEngineChirho::new_chirho();

        // Search for "God" in sample verses
        let results_chirho = engine_chirho.search_chirho("God", 10);

        // Should find at least one result (Genesis 1:1 has "God")
        assert!(!results_chirho.is_empty(), "Should find at least one result for 'God'");

        // Check that results contain the search term
        for (ref_chirho, text_chirho) in &results_chirho {
            assert!(
                text_chirho.to_lowercase().contains("god"),
                "Result '{}' should contain 'god': {}",
                ref_chirho,
                text_chirho
            );
        }
    }

    #[test]
    fn test_search_case_insensitive_chirho() {
        let engine_chirho = bible_engine_chirho::BibleEngineChirho::new_chirho();

        // Search should be case insensitive
        let results_upper_chirho = engine_chirho.search_chirho("GOD", 10);
        let results_lower_chirho = engine_chirho.search_chirho("god", 10);

        // Both should return results
        assert!(!results_upper_chirho.is_empty());
        assert!(!results_lower_chirho.is_empty());
    }

    #[test]
    fn test_search_no_results_chirho() {
        let engine_chirho = bible_engine_chirho::BibleEngineChirho::new_chirho();

        // Search for something that won't be found
        let results_chirho = engine_chirho.search_chirho("xyzabc123nonsense", 10);

        // Should return empty
        assert!(results_chirho.is_empty());
    }

    #[test]
    fn test_bookmarks_chirho() {
        use tempfile::tempdir;

        let temp_dir_chirho = tempdir().unwrap();
        let db_path_chirho = temp_dir_chirho.path().join("test_bookmarks.db");

        let conn_chirho = database_chirho::init_database_chirho(&db_path_chirho).unwrap();

        // Initially no bookmarks
        let bookmarks_chirho = database_chirho::get_all_bookmarks_chirho(&conn_chirho).unwrap();
        assert!(bookmarks_chirho.is_empty());

        // Add a bookmark
        let id_chirho = database_chirho::add_bookmark_chirho(
            &conn_chirho,
            "KJV",
            "John",
            3,
            16,
            Some("John 3:16 - God's Love")
        ).unwrap();
        assert!(id_chirho > 0);

        // Verify bookmark exists
        let bookmarks_chirho = database_chirho::get_all_bookmarks_chirho(&conn_chirho).unwrap();
        assert_eq!(bookmarks_chirho.len(), 1);
        assert_eq!(bookmarks_chirho[0].book_chirho, "John");
        assert_eq!(bookmarks_chirho[0].chapter_chirho, 3);
        assert_eq!(bookmarks_chirho[0].verse_chirho, 16);

        // Check is_bookmarked
        assert!(database_chirho::is_bookmarked_chirho(&conn_chirho, "KJV", "John", 3, 16));
        assert!(!database_chirho::is_bookmarked_chirho(&conn_chirho, "KJV", "John", 3, 17));

        // Get bookmark ID
        let found_id_chirho = database_chirho::get_bookmark_id_chirho(&conn_chirho, "KJV", "John", 3, 16);
        assert_eq!(found_id_chirho, Some(id_chirho));

        // Update bookmark label
        database_chirho::update_bookmark_label_chirho(&conn_chirho, id_chirho, "Updated Label").unwrap();
        let bookmarks_chirho = database_chirho::get_all_bookmarks_chirho(&conn_chirho).unwrap();
        assert_eq!(bookmarks_chirho[0].label_chirho, Some("Updated Label".to_string()));

        // Remove bookmark
        database_chirho::remove_bookmark_chirho(&conn_chirho, id_chirho).unwrap();
        let bookmarks_chirho = database_chirho::get_all_bookmarks_chirho(&conn_chirho).unwrap();
        assert!(bookmarks_chirho.is_empty());
    }

    #[test]
    fn test_notes_chirho() {
        use tempfile::tempdir;

        let temp_dir_chirho = tempdir().unwrap();
        let db_path_chirho = temp_dir_chirho.path().join("test_notes.db");

        let conn_chirho = database_chirho::init_database_chirho(&db_path_chirho).unwrap();

        // Initially no notes
        let notes_chirho = database_chirho::get_all_notes_chirho(&conn_chirho).unwrap();
        assert!(notes_chirho.is_empty());

        // Save a note
        database_chirho::save_note_chirho(
            &conn_chirho,
            "KJV",
            "John",
            3,
            16,
            "This is the most famous verse in the Bible!"
        ).unwrap();

        // Get the note
        let note_chirho = database_chirho::get_note_chirho(&conn_chirho, "KJV", "John", 3, 16);
        assert!(note_chirho.is_some());
        let note_chirho = note_chirho.unwrap();
        assert_eq!(note_chirho.content_chirho, "This is the most famous verse in the Bible!");

        // Update the note
        database_chirho::save_note_chirho(
            &conn_chirho,
            "KJV",
            "John",
            3,
            16,
            "Updated: God's love for the world."
        ).unwrap();

        let note_chirho = database_chirho::get_note_chirho(&conn_chirho, "KJV", "John", 3, 16).unwrap();
        assert_eq!(note_chirho.content_chirho, "Updated: God's love for the world.");

        // Search notes
        let search_results_chirho = database_chirho::search_notes_chirho(&conn_chirho, "love").unwrap();
        assert_eq!(search_results_chirho.len(), 1);

        // Get all notes
        let all_notes_chirho = database_chirho::get_all_notes_chirho(&conn_chirho).unwrap();
        assert_eq!(all_notes_chirho.len(), 1);

        // Delete the note
        database_chirho::delete_note_chirho(&conn_chirho, "KJV", "John", 3, 16).unwrap();
        let note_chirho = database_chirho::get_note_chirho(&conn_chirho, "KJV", "John", 3, 16);
        assert!(note_chirho.is_none());
    }

    #[test]
    fn test_highlight_colors_chirho() {
        use tempfile::tempdir;

        let temp_dir_chirho = tempdir().unwrap();
        let db_path_chirho = temp_dir_chirho.path().join("test_highlight_colors.db");

        let conn_chirho = database_chirho::init_database_chirho(&db_path_chirho).unwrap();

        // Add highlight with specific color
        database_chirho::add_highlight_with_color_chirho(
            &conn_chirho,
            "KJV",
            "Genesis",
            1,
            1,
            "green"
        ).unwrap();

        // Get highlights with colors
        let highlights_chirho = database_chirho::get_highlights_with_colors_chirho(
            &conn_chirho,
            "KJV",
            "Genesis",
            1
        ).unwrap();

        assert_eq!(highlights_chirho.len(), 1);
        assert_eq!(highlights_chirho[0].verse_chirho, 1);
        assert_eq!(highlights_chirho[0].color_chirho, "green");

        // Change color (upsert)
        database_chirho::add_highlight_with_color_chirho(
            &conn_chirho,
            "KJV",
            "Genesis",
            1,
            1,
            "blue"
        ).unwrap();

        let highlights_chirho = database_chirho::get_highlights_with_colors_chirho(
            &conn_chirho,
            "KJV",
            "Genesis",
            1
        ).unwrap();

        assert_eq!(highlights_chirho.len(), 1);
        assert_eq!(highlights_chirho[0].color_chirho, "blue");
    }

    #[test]
    fn test_reading_history_chirho() {
        use tempfile::tempdir;

        let temp_dir_chirho = tempdir().unwrap();
        let db_path_chirho = temp_dir_chirho.path().join("test_history.db");

        let conn_chirho = database_chirho::init_database_chirho(&db_path_chirho).unwrap();

        // Initially no history
        let history_chirho = database_chirho::get_recent_history_chirho(&conn_chirho, 10).unwrap();
        assert!(history_chirho.is_empty());

        // Add some history entries
        database_chirho::add_reading_history_chirho(&conn_chirho, "KJV", "Genesis", 1).unwrap();
        database_chirho::add_reading_history_chirho(&conn_chirho, "KJV", "John", 3).unwrap();
        database_chirho::add_reading_history_chirho(&conn_chirho, "KJV", "Psalms", 23).unwrap();

        // Get recent history
        let history_chirho = database_chirho::get_recent_history_chirho(&conn_chirho, 10).unwrap();
        assert_eq!(history_chirho.len(), 3);

        // Verify all entries are present (order may vary due to timestamp grouping)
        let books_chirho: Vec<&str> = history_chirho.iter().map(|h_chirho| h_chirho.book_chirho.as_str()).collect();
        assert!(books_chirho.contains(&"Genesis"));
        assert!(books_chirho.contains(&"John"));
        assert!(books_chirho.contains(&"Psalms"));

        // Test limit
        let limited_chirho = database_chirho::get_recent_history_chirho(&conn_chirho, 2).unwrap();
        assert_eq!(limited_chirho.len(), 2);
    }

    #[test]
    fn test_statistics_chirho() {
        use tempfile::tempdir;

        let temp_dir_chirho = tempdir().unwrap();
        let db_path_chirho = temp_dir_chirho.path().join("test_stats.db");

        let conn_chirho = database_chirho::init_database_chirho(&db_path_chirho).unwrap();

        // Initially all stats should be zero
        let stats_chirho = database_chirho::get_statistics_chirho(&conn_chirho).unwrap();
        assert_eq!(stats_chirho.total_chapters_read_chirho, 0);
        assert_eq!(stats_chirho.unique_chapters_read_chirho, 0);
        assert_eq!(stats_chirho.highlight_count_chirho, 0);
        assert_eq!(stats_chirho.note_count_chirho, 0);
        assert_eq!(stats_chirho.bookmark_count_chirho, 0);
        assert_eq!(stats_chirho.current_streak_chirho, 0);
        assert_eq!(stats_chirho.longest_streak_chirho, 0);

        // Add some reading history
        database_chirho::add_reading_history_chirho(&conn_chirho, "KJV", "Genesis", 1).unwrap();
        database_chirho::add_reading_history_chirho(&conn_chirho, "KJV", "Genesis", 1).unwrap(); // duplicate
        database_chirho::add_reading_history_chirho(&conn_chirho, "KJV", "John", 3).unwrap();

        // Add highlights, notes, bookmarks
        database_chirho::toggle_highlight_chirho(&conn_chirho, "KJV", "Genesis", 1, 1).unwrap();
        database_chirho::toggle_highlight_chirho(&conn_chirho, "KJV", "John", 3, 16).unwrap();
        database_chirho::save_note_chirho(&conn_chirho, "KJV", "Genesis", 1, 1, "Test note").unwrap();
        database_chirho::add_bookmark_chirho(&conn_chirho, "KJV", "John", 3, 16, Some("Favorite")).unwrap();

        // Verify stats
        let stats_chirho = database_chirho::get_statistics_chirho(&conn_chirho).unwrap();
        assert_eq!(stats_chirho.total_chapters_read_chirho, 3); // 3 history entries
        assert_eq!(stats_chirho.unique_chapters_read_chirho, 2); // 2 unique chapters
        assert_eq!(stats_chirho.highlight_count_chirho, 2);
        assert_eq!(stats_chirho.note_count_chirho, 1);
        assert_eq!(stats_chirho.bookmark_count_chirho, 1);
        assert_eq!(stats_chirho.books_started_chirho, 2); // Genesis and John
    }

    #[test]
    fn test_format_duration_chirho() {
        assert_eq!(database_chirho::format_duration_chirho(0), "0m");
        assert_eq!(database_chirho::format_duration_chirho(60), "1m");
        assert_eq!(database_chirho::format_duration_chirho(3600), "1h 0m");
        assert_eq!(database_chirho::format_duration_chirho(3660), "1h 1m");
        assert_eq!(database_chirho::format_duration_chirho(7200), "2h 0m");
        assert_eq!(database_chirho::format_duration_chirho(7245), "2h 0m"); // 7245 / 60 % 60 = 0
    }

    #[test]
    fn test_journal_entries_chirho() {
        use tempfile::tempdir;

        let temp_dir_chirho = tempdir().unwrap();
        let db_path_chirho = temp_dir_chirho.path().join("test_journal.db");

        let conn_chirho = database_chirho::init_database_chirho(&db_path_chirho).unwrap();

        // Initially no entries
        let entries_chirho = database_chirho::get_all_journal_entries_chirho(&conn_chirho).unwrap();
        assert!(entries_chirho.is_empty());

        // Create an entry
        let id_chirho = database_chirho::create_journal_entry_chirho(
            &conn_chirho,
            "Test Entry",
            "This is the content of my journal entry.",
            Some("John 3:16"),
            Some("faith,love"),
        ).unwrap();
        assert!(id_chirho > 0);

        // Verify entry exists
        let entries_chirho = database_chirho::get_all_journal_entries_chirho(&conn_chirho).unwrap();
        assert_eq!(entries_chirho.len(), 1);
        assert_eq!(entries_chirho[0].title_chirho, "Test Entry");
        assert_eq!(entries_chirho[0].verse_ref_chirho, Some("John 3:16".to_string()));

        // Update entry
        database_chirho::update_journal_entry_chirho(
            &conn_chirho,
            id_chirho,
            "Updated Title",
            "Updated content.",
            Some("Romans 8:28"),
            Some("hope"),
        ).unwrap();

        let entries_chirho = database_chirho::get_all_journal_entries_chirho(&conn_chirho).unwrap();
        assert_eq!(entries_chirho[0].title_chirho, "Updated Title");
        assert_eq!(entries_chirho[0].verse_ref_chirho, Some("Romans 8:28".to_string()));

        // Delete entry
        database_chirho::delete_journal_entry_chirho(&conn_chirho, id_chirho).unwrap();
        let entries_chirho = database_chirho::get_all_journal_entries_chirho(&conn_chirho).unwrap();
        assert!(entries_chirho.is_empty());
    }

    #[test]
    fn test_parse_reference_full_name_chirho() {
        let result_chirho = parse_reference_chirho("John 3:16");
        assert!(result_chirho.is_some());
        let (book_chirho, chapter_chirho, verse_chirho) = result_chirho.unwrap();
        assert_eq!(book_chirho, "John");
        assert_eq!(chapter_chirho, 3);
        assert_eq!(verse_chirho, 16);
    }

    #[test]
    fn test_parse_reference_abbreviated_chirho() {
        let result_chirho = parse_reference_chirho("Gen 1:1");
        assert!(result_chirho.is_some());
        let (book_chirho, chapter_chirho, verse_chirho) = result_chirho.unwrap();
        assert_eq!(book_chirho, "Genesis");
        assert_eq!(chapter_chirho, 1);
        assert_eq!(verse_chirho, 1);
    }

    #[test]
    fn test_parse_reference_short_abbreviation_chirho() {
        let result_chirho = parse_reference_chirho("Jn 3:16");
        assert!(result_chirho.is_some());
        let (book_chirho, chapter_chirho, verse_chirho) = result_chirho.unwrap();
        assert_eq!(book_chirho, "John");
        assert_eq!(chapter_chirho, 3);
        assert_eq!(verse_chirho, 16);
    }

    #[test]
    fn test_parse_reference_no_verse_chirho() {
        let result_chirho = parse_reference_chirho("Psalms 23");
        assert!(result_chirho.is_some());
        let (book_chirho, chapter_chirho, verse_chirho) = result_chirho.unwrap();
        assert_eq!(book_chirho, "Psalms");
        assert_eq!(chapter_chirho, 23);
        assert_eq!(verse_chirho, 1); // Default to verse 1
    }

    #[test]
    fn test_parse_reference_numbered_book_chirho() {
        let result_chirho = parse_reference_chirho("1Cor 13:4");
        assert!(result_chirho.is_some());
        let (book_chirho, chapter_chirho, verse_chirho) = result_chirho.unwrap();
        assert_eq!(book_chirho, "1 Corinthians");
        assert_eq!(chapter_chirho, 13);
        assert_eq!(verse_chirho, 4);
    }

    #[test]
    fn test_parse_reference_revelation_chirho() {
        let result_chirho = parse_reference_chirho("Rev 21:4");
        assert!(result_chirho.is_some());
        let (book_chirho, chapter_chirho, verse_chirho) = result_chirho.unwrap();
        assert_eq!(book_chirho, "Revelation");
        assert_eq!(chapter_chirho, 21);
        assert_eq!(verse_chirho, 4);
    }

    #[test]
    fn test_parse_reference_verse_range_chirho() {
        // Test verse range like "Gen 1:1-10" - should navigate to start verse
        let result_chirho = parse_reference_chirho("Gen 1:1-10");
        assert!(result_chirho.is_some());
        let (book_chirho, chapter_chirho, verse_chirho) = result_chirho.unwrap();
        assert_eq!(book_chirho, "Genesis");
        assert_eq!(chapter_chirho, 1);
        assert_eq!(verse_chirho, 1);

        // Test another range
        let result_chirho = parse_reference_chirho("John 3:16-21");
        assert!(result_chirho.is_some());
        let (book_chirho, chapter_chirho, verse_chirho) = result_chirho.unwrap();
        assert_eq!(book_chirho, "John");
        assert_eq!(chapter_chirho, 3);
        assert_eq!(verse_chirho, 16);
    }

    #[test]
    fn test_parse_reference_case_insensitive_chirho() {
        let result_chirho = parse_reference_chirho("JOHN 3:16");
        assert!(result_chirho.is_some());
        let (book_chirho, _chapter_chirho, _verse_chirho) = result_chirho.unwrap();
        assert_eq!(book_chirho, "John");
    }

    #[test]
    fn test_format_timestamp_chirho() {
        // Test basic timestamp formatting
        let result_chirho = format_timestamp_chirho("2024-01-15 10:30:00");
        assert_eq!(result_chirho, "2024-01-15");

        // Test empty timestamp
        let result_chirho = format_timestamp_chirho("");
        assert_eq!(result_chirho, "");
    }

    #[test]
    fn test_bible_books_order_chirho() {
        // Verify first and last books
        assert_eq!(BIBLE_BOOKS_CHIRHO[0].0, "Genesis");
        assert_eq!(BIBLE_BOOKS_CHIRHO[65].0, "Revelation");

        // Verify OT/NT boundary
        assert_eq!(BIBLE_BOOKS_CHIRHO[38].0, "Malachi"); // Last OT book
        assert_eq!(BIBLE_BOOKS_CHIRHO[39].0, "Matthew"); // First NT book
    }

    #[test]
    fn test_parse_reference_mixed_case_chirho() {
        // Test various case combinations
        let result_chirho = parse_reference_chirho("GeNeSiS 1:1");
        assert!(result_chirho.is_some());
        let (book_chirho, chapter_chirho, verse_chirho) = result_chirho.unwrap();
        assert_eq!(book_chirho, "Genesis");
        assert_eq!(chapter_chirho, 1);
        assert_eq!(verse_chirho, 1);
    }

    #[test]
    fn test_parse_reference_invalid_chirho() {
        // Test invalid references
        assert!(parse_reference_chirho("").is_none());
        assert!(parse_reference_chirho("   ").is_none());
        assert!(parse_reference_chirho("NotABook 1:1").is_none());
    }

    #[test]
    fn test_quick_nav_search_chirho() {
        use tempfile::tempdir;

        let temp_dir_chirho = tempdir().unwrap();
        let db_path_chirho = temp_dir_chirho.path().join("test_quicknav.db");

        let conn_chirho = database_chirho::init_database_chirho(&db_path_chirho).unwrap();

        // Add some reading history
        database_chirho::add_reading_history_chirho(&conn_chirho, "KJV", "Genesis", 1).unwrap();
        database_chirho::add_reading_history_chirho(&conn_chirho, "KJV", "John", 3).unwrap();

        // Test empty query returns history
        let results_chirho = quick_nav_search_chirho("", &conn_chirho);
        assert_eq!(results_chirho.len(), 2);

        // Test book name search
        let results_chirho = quick_nav_search_chirho("gen", &conn_chirho);
        assert!(!results_chirho.is_empty());
        // Should find Genesis
        assert!(results_chirho.iter().any(|r_chirho| r_chirho.book_chirho.as_str() == "Genesis"));

        // Test verse reference search
        let results_chirho = quick_nav_search_chirho("John 3", &conn_chirho);
        assert!(!results_chirho.is_empty());
        assert!(results_chirho.iter().any(|r_chirho| r_chirho.book_chirho.as_str() == "John" && r_chirho.chapter_chirho == 3));

        // Test fuzzy matching - should find Psalms
        let results_chirho = quick_nav_search_chirho("psa", &conn_chirho);
        assert!(!results_chirho.is_empty());
        assert!(results_chirho.iter().any(|r_chirho| r_chirho.book_chirho.as_str() == "Psalms"));
    }

    #[test]
    fn test_quick_nav_search_edge_cases_chirho() {
        use tempfile::tempdir;

        let temp_dir_chirho = tempdir().unwrap();
        let db_path_chirho = temp_dir_chirho.path().join("test_quicknav_edge.db");

        let conn_chirho = database_chirho::init_database_chirho(&db_path_chirho).unwrap();

        // Test numbered book search
        let results_chirho = quick_nav_search_chirho("1 cor", &conn_chirho);
        assert!(!results_chirho.is_empty());
        assert!(results_chirho.iter().any(|r_chirho| r_chirho.book_chirho.as_str() == "1 Corinthians"));

        // Test single letter (should find nothing specific but not crash)
        let _results_chirho = quick_nav_search_chirho("x", &conn_chirho);
        // May be empty or find Exodus

        // Test exact book name
        let results_chirho = quick_nav_search_chirho("revelation", &conn_chirho);
        assert!(!results_chirho.is_empty());
        assert!(results_chirho.iter().any(|r_chirho| r_chirho.book_chirho.as_str() == "Revelation"));

        // Test case insensitivity
        let results_chirho = quick_nav_search_chirho("JOHN", &conn_chirho);
        assert!(!results_chirho.is_empty());
        assert!(results_chirho.iter().any(|r_chirho| r_chirho.book_chirho.as_str() == "John"));
    }

    #[test]
    fn test_copy_format_helper_chirho() {
        // Test different copy format scenarios
        let text_chirho = "For God so loved the world";
        let reference_chirho = "John 3:16";

        // Format 0: text - reference
        let formatted_0_chirho = format!("{} - {}", text_chirho, reference_chirho);
        assert_eq!(formatted_0_chirho, "For God so loved the world - John 3:16");

        // Format 1: reference: text
        let formatted_1_chirho = format!("{}: {}", reference_chirho, text_chirho);
        assert_eq!(formatted_1_chirho, "John 3:16: For God so loved the world");

        // Format 2: text only
        let formatted_2_chirho = text_chirho.to_string();
        assert_eq!(formatted_2_chirho, "For God so loved the world");
    }

    #[test]
    fn test_multi_verse_format_chirho() {
        // Test multi-verse formatting
        let verses_chirho = vec![
            (1, "In the beginning God created the heaven and the earth."),
            (2, "And the earth was without form, and void."),
            (3, "And God said, Let there be light: and there was light."),
        ];

        // With verse numbers
        let text_with_numbers_chirho: Vec<String> = verses_chirho.iter()
            .map(|(num_chirho, text_chirho)| format!("{}. {}", num_chirho, text_chirho))
            .collect();
        let combined_chirho = text_with_numbers_chirho.join(" ");
        assert!(combined_chirho.contains("1. In the beginning"));
        assert!(combined_chirho.contains("2. And the earth"));
        assert!(combined_chirho.contains("3. And God said"));

        // Without verse numbers
        let text_without_numbers_chirho: Vec<String> = verses_chirho.iter()
            .map(|(_num_chirho, text_chirho)| text_chirho.to_string())
            .collect();
        let combined_no_nums_chirho = text_without_numbers_chirho.join(" ");
        assert!(!combined_no_nums_chirho.contains("1."));
        assert!(combined_no_nums_chirho.contains("In the beginning"));
    }

    #[test]
    fn test_prayer_requests_chirho() {
        use tempfile::tempdir;

        let temp_dir_chirho = tempdir().unwrap();
        let db_path_chirho = temp_dir_chirho.path().join("test_prayer.db");

        let conn_chirho = database_chirho::init_database_chirho(&db_path_chirho).unwrap();

        // Initially no prayer requests
        let requests_chirho = database_chirho::get_all_prayer_requests_chirho(&conn_chirho).unwrap();
        assert!(requests_chirho.is_empty());

        // Check counts
        let (active_chirho, total_chirho) = database_chirho::get_prayer_counts_chirho(&conn_chirho);
        assert_eq!(active_chirho, 0);
        assert_eq!(total_chirho, 0);

        // Create a prayer request
        let id_chirho = database_chirho::create_prayer_request_chirho(
            &conn_chirho,
            "Health for family",
            Some("Please pray for healing"),
            Some("James 5:14"),
            Some("Health"),
        ).unwrap();
        assert!(id_chirho > 0);

        // Create another prayer request
        let id2_chirho = database_chirho::create_prayer_request_chirho(
            &conn_chirho,
            "Guidance for job",
            Some("Seeking wisdom for career decisions"),
            Some("Proverbs 3:5-6"),
            Some("Career"),
        ).unwrap();
        assert!(id2_chirho > 0);

        // Verify requests exist
        let requests_chirho = database_chirho::get_all_prayer_requests_chirho(&conn_chirho).unwrap();
        assert_eq!(requests_chirho.len(), 2);

        // Check counts
        let (active_chirho, total_chirho) = database_chirho::get_prayer_counts_chirho(&conn_chirho);
        assert_eq!(active_chirho, 2);
        assert_eq!(total_chirho, 2);

        // Mark first prayer as answered
        database_chirho::mark_prayer_answered_chirho(&conn_chirho, id_chirho).unwrap();

        let requests_chirho = database_chirho::get_all_prayer_requests_chirho(&conn_chirho).unwrap();
        // Active prayers should come first
        assert!(!requests_chirho[0].is_answered_chirho);
        assert!(requests_chirho[1].is_answered_chirho);

        // Check counts after marking answered
        let (active_chirho, total_chirho) = database_chirho::get_prayer_counts_chirho(&conn_chirho);
        assert_eq!(active_chirho, 1);
        assert_eq!(total_chirho, 2);

        // Delete a prayer request
        database_chirho::delete_prayer_request_chirho(&conn_chirho, id2_chirho).unwrap();
        let requests_chirho = database_chirho::get_all_prayer_requests_chirho(&conn_chirho).unwrap();
        assert_eq!(requests_chirho.len(), 1);
        assert_eq!(requests_chirho[0].title_chirho, "Health for family");

        // Check final counts
        let (active_chirho, total_chirho) = database_chirho::get_prayer_counts_chirho(&conn_chirho);
        assert_eq!(active_chirho, 0);  // The remaining one is answered
        assert_eq!(total_chirho, 1);
    }

    #[test]
    fn test_commentary_chirho() {
        // Test known commentary entries
        let (content_chirho, module_chirho) = get_sample_commentary_chirho("Genesis 1:1");
        assert!(content_chirho.contains("In the beginning"));
        assert!(content_chirho.contains("Hebrew"));
        assert_eq!(module_chirho, "MHCC");

        let (content_chirho, module_chirho) = get_sample_commentary_chirho("John 3:16");
        assert!(content_chirho.contains("agape"));
        assert!(content_chirho.contains("gospel in miniature"));
        assert_eq!(module_chirho, "MHCC");

        let (content_chirho, module_chirho) = get_sample_commentary_chirho("Psalm 23:1");
        assert!(content_chirho.contains("shepherd"));
        assert_eq!(module_chirho, "MHCC");

        // Test unknown verse
        let (content_chirho, module_chirho) = get_sample_commentary_chirho("Unknown 99:99");
        assert!(content_chirho.contains("not found"));
        assert!(module_chirho.is_empty());
    }

    #[test]
    fn test_lexicon_chirho() {
        // Test known lexicon entries
        let (title_chirho, content_chirho) = get_sample_lexicon_entry_chirho("love");
        assert!(title_chirho.contains("agape"));
        assert!(content_chirho.contains("G26"));
        assert!(content_chirho.contains("unconditional"));

        let (title_chirho, content_chirho) = get_sample_lexicon_entry_chirho("logos");
        assert!(title_chirho.contains("logos"));
        assert!(content_chirho.contains("G3056"));
        assert!(content_chirho.contains("Word"));

        // Test case insensitivity
        let (title_chirho, content_chirho) = get_sample_lexicon_entry_chirho("FAITH");
        assert!(title_chirho.contains("pistis"));
        assert!(content_chirho.contains("G4102"));

        // Test Greek input
        let (title_chirho, content_chirho) = get_sample_lexicon_entry_chirho("θεός");
        assert!(title_chirho.contains("theos"));
        assert!(content_chirho.contains("G2316"));

        // Test unknown word
        let (title_chirho, content_chirho) = get_sample_lexicon_entry_chirho("unknownword");
        assert_eq!(title_chirho, "unknownword");
        assert!(content_chirho.contains("not found"));
    }

    #[test]
    fn test_versification_chirho() {
        use tempfile::tempdir;

        let temp_dir_chirho = tempdir().unwrap();
        let db_path_chirho = temp_dir_chirho.path().join("test_versification.db");

        let conn_chirho = database_chirho::init_database_chirho(&db_path_chirho).unwrap();

        // Initially no versification setting
        let setting_chirho = database_chirho::get_setting_chirho(&conn_chirho, "versification_system");
        assert!(setting_chirho.is_none());

        // Set versification to Catholic (index 1)
        database_chirho::set_setting_chirho(&conn_chirho, "versification_system", "1").unwrap();
        let setting_chirho = database_chirho::get_setting_chirho(&conn_chirho, "versification_system");
        assert_eq!(setting_chirho, Some("1".to_string()));

        // Change to LXX (index 3)
        database_chirho::set_setting_chirho(&conn_chirho, "versification_system", "3").unwrap();
        let setting_chirho = database_chirho::get_setting_chirho(&conn_chirho, "versification_system");
        assert_eq!(setting_chirho, Some("3".to_string()));

        // Versification names for reference
        let versification_names_chirho = ["KJV", "Catholic", "Orthodox", "LXX", "Vulgate", "Luther"];
        assert_eq!(versification_names_chirho.len(), 6);
        assert_eq!(versification_names_chirho[0], "KJV");
        assert_eq!(versification_names_chirho[3], "LXX");
    }

    #[test]
    fn test_interlinear_chirho() {
        // Test Genesis 1:1 interlinear data
        let genesis_words_chirho = get_sample_interlinear_chirho("Genesis 1:1");
        assert!(!genesis_words_chirho.is_empty());
        assert_eq!(genesis_words_chirho.len(), 7); // 7 words in Genesis 1:1 Hebrew

        // First word should be "בְּרֵאשִׁ֖ית" (bereshit)
        assert!(genesis_words_chirho[0].original_chirho.contains("בְּרֵאשִׁ֖ית"));
        assert!(genesis_words_chirho[0].is_hebrew_chirho);
        assert_eq!(genesis_words_chirho[0].strongs_chirho.as_str(), "H7225");
        assert!(genesis_words_chirho[0].gloss_chirho.contains("beginning"));

        // Third word should be Elohim
        assert!(genesis_words_chirho[2].original_chirho.contains("אֱלֹהִ֑ים"));
        assert_eq!(genesis_words_chirho[2].strongs_chirho.as_str(), "H430");
        assert!(genesis_words_chirho[2].gloss_chirho.contains("God"));

        // Test John 3:16 interlinear data
        let john_words_chirho = get_sample_interlinear_chirho("John 3:16");
        assert!(!john_words_chirho.is_empty());
        assert!(!john_words_chirho[0].is_hebrew_chirho); // Greek, not Hebrew

        // Should have "ἠγάπησεν" (loved)
        let loved_word_chirho = john_words_chirho.iter().find(|w| w.gloss_chirho.contains("loved"));
        assert!(loved_word_chirho.is_some());
        assert_eq!(loved_word_chirho.unwrap().strongs_chirho.as_str(), "G25");

        // Test unknown verse
        let unknown_words_chirho = get_sample_interlinear_chirho("Unknown 99:99");
        assert_eq!(unknown_words_chirho.len(), 1);
        assert!(unknown_words_chirho[0].gloss_chirho.contains("not available"));
    }
}
