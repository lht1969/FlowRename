// Undo Manager - Persistent undo history management
// Saves/loads undo history to a JSON file so it survives app restarts

use crate::commands::rename_commands::UndoEntry;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Maximum number of undo entries to keep in history
const MAX_UNDO_ENTRIES: usize = 50;

/// Maximum age of undo entries in days before they are auto-cleaned
const MAX_UNDO_AGE_DAYS: i64 = 30;

/// Undo Manager handles persistent storage of undo history
/// 
/// The undo history is saved to a JSON file in the app's data directory.
/// This allows users to undo rename operations even after restarting the app.
pub struct UndoManager {
    /// Path to the undo history file
    history_path: PathBuf,
}

impl UndoManager {
    /// Create a new UndoManager with the specified data directory
    /// 
    /// # Arguments
    /// * `data_dir` - Directory where the undo history file will be stored
    pub fn new(data_dir: &Path) -> Self {
        // Ensure the data directory exists
        if !data_dir.exists() {
            if let Err(e) = fs::create_dir_all(data_dir) {
                log::warn!("Failed to create undo data directory: {}", e);
            }
        }

        let history_path = data_dir.join("undo_history.json");
        log::info!("Undo history path: {}", history_path.display());

        Self { history_path }
    }

    /// Load undo history from disk
    /// Returns an empty vector if the file doesn't exist or is corrupted
    pub fn load(&self) -> Vec<UndoEntry> {
        if !self.history_path.exists() {
            log::info!("No undo history file found, starting fresh");
            return Vec::new();
        }

        match fs::read_to_string(&self.history_path) {
            Ok(content) => {
                match serde_json::from_str::<Vec<UndoEntry>>(&content) {
                    Ok(entries) => {
                        log::info!("Loaded {} undo entries from disk", entries.len());
                        // Clean up expired entries on load
                        self.clean_expired_entries(entries)
                    }
                    Err(e) => {
                        log::warn!("Failed to parse undo history: {}, starting fresh", e);
                        // Backup corrupted file before resetting
                        self.backup_corrupted_file();
                        Vec::new()
                    }
                }
            }
            Err(e) => {
                log::warn!("Failed to read undo history: {}, starting fresh", e);
                Vec::new()
            }
        }
    }

    /// Save undo history to disk
    /// Only the most recent MAX_UNDO_ENTRIES entries are kept
    pub fn save(&self, entries: &[UndoEntry]) -> Result<()> {
        // Trim to max entries (keep the most recent ones)
        let entries_to_save = if entries.len() > MAX_UNDO_ENTRIES {
            &entries[entries.len() - MAX_UNDO_ENTRIES..]
        } else {
            entries
        };

        let json = serde_json::to_string_pretty(entries_to_save)
            .context("Failed to serialize undo history")?;

        // Write atomically: write to temp file first, then rename
        let temp_path = self.history_path.with_extension("json.tmp");

        fs::write(&temp_path, &json)
            .context("Failed to write undo history temp file")?;

        fs::rename(&temp_path, &self.history_path)
            .context("Failed to rename undo history temp file")?;

        log::debug!("Saved {} undo entries to disk", entries_to_save.len());
        Ok(())
    }

    /// Add a new undo entry and save to disk
    /// Automatically trims old entries if the history exceeds the limit
    pub fn add_entry(&self, entries: &mut Vec<UndoEntry>, entry: UndoEntry) -> Result<()> {
        entries.push(entry);

        // Trim old entries if exceeding the limit
        while entries.len() > MAX_UNDO_ENTRIES {
            entries.remove(0);
        }

        self.save(entries)
    }

    /// Remove and return the last undo entry, saving the updated history
    pub fn pop_entry(&self, entries: &mut Vec<UndoEntry>) -> Option<UndoEntry> {
        let entry = entries.pop()?;
        
        // Save updated history (ignore errors, the pop still succeeded in memory)
        if let Err(e) = self.save(entries) {
            log::warn!("Failed to save undo history after pop: {}", e);
        }

        Some(entry)
    }

    /// Clear all undo history
    pub fn clear(&self, entries: &mut Vec<UndoEntry>) -> Result<()> {
        entries.clear();
        self.save(entries)
    }

    /// Get the number of undo entries
    pub fn count(&self, entries: &[UndoEntry]) -> usize {
        entries.len()
    }

    /// Check if there is any undo history available
    pub fn has_history(entries: &[UndoEntry]) -> bool {
        !entries.is_empty()
    }

    /// Clean up entries that are older than MAX_UNDO_AGE_DAYS
    fn clean_expired_entries(&self, entries: Vec<UndoEntry>) -> Vec<UndoEntry> {
        let cutoff = chrono::Utc::now() - chrono::Duration::days(MAX_UNDO_AGE_DAYS);
        let cutoff_str = cutoff.to_rfc3339();

        let original_count = entries.len();
        let valid_entries: Vec<UndoEntry> = entries
            .into_iter()
            .filter(|entry| entry.created_at > cutoff_str)
            .collect();

        let removed_count = original_count.saturating_sub(valid_entries.len());
        if removed_count > 0 {
            log::info!("Cleaned {} expired undo entries (older than {} days)", 
                removed_count, MAX_UNDO_AGE_DAYS);
            // Save the cleaned history
            if let Err(e) = self.save(&valid_entries) {
                log::warn!("Failed to save cleaned undo history: {}", e);
            }
        }

        valid_entries
    }

    /// Backup a corrupted undo history file for debugging
    fn backup_corrupted_file(&self) {
        let backup_path = self.history_path.with_extension("json.corrupted");
        if let Err(e) = fs::rename(&self.history_path, &backup_path) {
            log::warn!("Failed to backup corrupted undo history: {}", e);
        } else {
            log::info!("Backed up corrupted undo history to {}", backup_path.display());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::rename_commands::UndoOperation;
    use tempfile::TempDir;

    /// Helper to create a test undo entry
    fn create_test_entry(id_suffix: &str, op_count: usize) -> UndoEntry {
        UndoEntry {
            id: format!("undo_test_{}", id_suffix),
            operations: (0..op_count)
                .map(|i| UndoOperation {
                    current_path: format!("test/file_{}.txt", i),
                    original_name: format!("original_{}.txt", i),
                })
                .collect(),
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    #[test]
    fn test_new_manager_creates_directory() {
        let temp_dir = TempDir::new().unwrap();
        let data_dir = temp_dir.path().join("undo_data");
        
        assert!(!data_dir.exists());
        
        let _manager = UndoManager::new(&data_dir);
        
        assert!(data_dir.exists());
    }

    #[test]
    fn test_load_empty_history() {
        let temp_dir = TempDir::new().unwrap();
        let manager = UndoManager::new(temp_dir.path());
        
        let entries = manager.load();
        assert!(entries.is_empty());
    }

    #[test]
    fn test_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let manager = UndoManager::new(temp_dir.path());
        
        let entries = vec![
            create_test_entry("1", 3),
            create_test_entry("2", 5),
        ];
        
        manager.save(&entries).unwrap();
        let loaded = manager.load();
        
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].id, "undo_test_1");
        assert_eq!(loaded[0].operations.len(), 3);
        assert_eq!(loaded[1].id, "undo_test_2");
        assert_eq!(loaded[1].operations.len(), 5);
    }

    #[test]
    fn test_add_entry() {
        let temp_dir = TempDir::new().unwrap();
        let manager = UndoManager::new(temp_dir.path());
        
        let mut entries = Vec::new();
        manager.add_entry(&mut entries, create_test_entry("1", 2)).unwrap();
        
        assert_eq!(entries.len(), 1);
        
        // Verify persistence
        let loaded = manager.load();
        assert_eq!(loaded.len(), 1);
    }

    #[test]
    fn test_pop_entry() {
        let temp_dir = TempDir::new().unwrap();
        let manager = UndoManager::new(temp_dir.path());
        
        let mut entries = vec![create_test_entry("1", 2), create_test_entry("2", 3)];
        manager.save(&entries).unwrap();
        
        let popped = manager.pop_entry(&mut entries);
        assert!(popped.is_some());
        assert_eq!(popped.unwrap().id, "undo_test_2");
        assert_eq!(entries.len(), 1);
        
        // Verify persistence
        let loaded = manager.load();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].id, "undo_test_1");
    }

    #[test]
    fn test_clear_history() {
        let temp_dir = TempDir::new().unwrap();
        let manager = UndoManager::new(temp_dir.path());
        
        let mut entries = vec![create_test_entry("1", 2)];
        manager.save(&entries).unwrap();
        
        manager.clear(&mut entries).unwrap();
        assert!(entries.is_empty());
        
        let loaded = manager.load();
        assert!(loaded.is_empty());
    }

    #[test]
    fn test_max_entries_limit() {
        let temp_dir = TempDir::new().unwrap();
        let manager = UndoManager::new(temp_dir.path());
        
        let mut entries = Vec::new();
        
        // Add more than MAX_UNDO_ENTRIES
        for i in 0..(MAX_UNDO_ENTRIES + 10) {
            manager.add_entry(&mut entries, create_test_entry(&i.to_string(), 1)).unwrap();
        }
        
        // Should be trimmed to MAX_UNDO_ENTRIES
        assert_eq!(entries.len(), MAX_UNDO_ENTRIES);
        
        // Verify the oldest entries were removed
        assert_eq!(entries[0].id, format!("undo_test_{}", 10));
        assert_eq!(entries.last().unwrap().id, format!("undo_test_{}", MAX_UNDO_ENTRIES + 9));
    }

    #[test]
    fn test_has_history() {
        let entries = vec![create_test_entry("1", 1)];
        assert!(UndoManager::has_history(&entries));
        
        let empty: Vec<UndoEntry> = Vec::new();
        assert!(!UndoManager::has_history(&empty));
    }

    #[test]
    fn test_corrupted_file_recovery() {
        let temp_dir = TempDir::new().unwrap();
        let manager = UndoManager::new(temp_dir.path());
        
        // Write invalid JSON to the history file
        let history_path = temp_dir.path().join("undo_history.json");
        fs::write(&history_path, "{invalid json content}").unwrap();
        
        // Should recover gracefully
        let entries = manager.load();
        assert!(entries.is_empty());
        
        // Corrupted file should be backed up
        let backup_path = temp_dir.path().join("undo_history.json.corrupted");
        assert!(backup_path.exists());
    }
}
