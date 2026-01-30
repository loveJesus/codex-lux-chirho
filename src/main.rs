// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

use std::rc::Rc;
use std::cell::RefCell;
use std::path::PathBuf;
use std::collections::HashSet;
use anyhow::Result;
use log::{info, warn, debug};
use rusqlite::{Connection, params};
use directories::ProjectDirs;

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

            -- Create indexes for faster lookups
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

    /// Get all highlights for a chapter
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
}

// ============================================================================
// Bible Engine Module
// ============================================================================

mod bible_engine_chirho {
    use super::*;
    use rsword_chirho::SwMgrChirho;

    /// Bible engine wrapping rsword_chirho
    pub struct BibleEngineChirho {
        manager_chirho: Option<SwMgrChirho>,
        current_module_chirho: String,
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
                                verses_chirho.push((
                                    verse_num_chirho.to_string(),
                                    text_chirho,
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
    }
}

// ============================================================================
// Sample Verses (for demo when no SWORD modules installed)
// ============================================================================

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

        // Get highlights and notes for this chapter
        let highlights_chirho = database_chirho::get_highlights_chirho(
            &self.db_conn_chirho,
            &self.current_module_chirho,
            &self.current_book_chirho,
            self.current_chapter_chirho,
        ).unwrap_or_default();

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
                VerseChirho {
                    reference_chirho: ref_chirho.into(),
                    text_chirho: text_chirho.into(),
                    is_highlighted_chirho: highlights_chirho.contains(&verse_num_chirho),
                    has_note_chirho: notes_chirho.contains(&verse_num_chirho),
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

    // Initialize books list
    let books_model_chirho: Vec<BookChirho> = BIBLE_BOOKS_CHIRHO
        .iter()
        .map(|(name_chirho, chapters_chirho)| BookChirho {
            name_chirho: (*name_chirho).into(),
            chapters_chirho: *chapters_chirho,
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
    }

    // Load initial verses
    {
        let verses_chirho = backend_chirho.borrow().get_verses_chirho();
        app_state_chirho.set_verses_chirho(Rc::new(slint::VecModel::from(verses_chirho)).into());
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

                let verses_chirho = backend_mut_chirho.get_verses_chirho();
                state_chirho.set_verses_chirho(Rc::new(slint::VecModel::from(verses_chirho)).into());

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

    // Set up search callback
    {
        let window_weak_chirho = main_window_chirho.as_weak();

        app_state_chirho.on_search_chirho(move |query_chirho| {
            info!("Search query: {}", query_chirho);

            if let Some(window_chirho) = window_weak_chirho.upgrade() {
                let state_chirho = window_chirho.global::<AppStateChirho>();

                // Demo search results - TODO: implement real search with Tantivy
                let results_chirho = vec![
                    VerseChirho {
                        reference_chirho: "John 3:16".into(),
                        text_chirho: "For God so loved the world, that he gave his only begotten Son...".into(),
                        is_highlighted_chirho: false,
                        has_note_chirho: false,
                    },
                    VerseChirho {
                        reference_chirho: "Romans 5:8".into(),
                        text_chirho: "But God commendeth his love toward us, in that, while we were yet sinners, Christ died for us.".into(),
                        is_highlighted_chirho: false,
                        has_note_chirho: false,
                    },
                    VerseChirho {
                        reference_chirho: "1 John 4:8".into(),
                        text_chirho: "He that loveth not knoweth not God; for God is love.".into(),
                        is_highlighted_chirho: false,
                        has_note_chirho: false,
                    },
                ];
                state_chirho.set_search_results_chirho(Rc::new(slint::VecModel::from(results_chirho)).into());
                state_chirho.set_status_message_chirho(format!("Found 3 results for '{}'", query_chirho).into());
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

    // Set initial status
    app_state_chirho.set_status_message_chirho("Welcome to Codex Lux - Your Bible Study Companion".into());

    // Run the application
    main_window_chirho.run()
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
}
