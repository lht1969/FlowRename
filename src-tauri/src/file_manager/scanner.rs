use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::metadata_reader::MetadataReader;
use crate::models::{FileItem, FileStatus};

/// File Manager - handles file system operations
/// Responsible for loading, scanning, and filtering files from the filesystem
pub struct FileManager;

impl FileManager {
    /// Load files from paths (supports both files and directories)
    ///
    /// # Arguments
    /// * `paths` - List of file or directory paths to load
    /// * `recursive` - Whether to scan directories recursively (true) or only top-level (false)
    /// * `extensions_filter` - Optional filter to only include files with certain extensions (e.g., vec!["jpg", "png"])
    ///                        Extensions should be lowercase without dots. Case-insensitive comparison.
    ///
    /// # Returns
    /// A `Result` containing a vector of `FileItem` objects representing all found and valid files.
    /// Returns an error if no valid paths are provided or all paths fail to load.
    ///
    /// # Examples
    /// ```ignore
    /// // Load a single file
    /// let files = FileManager::load_files(vec![path.to_path_buf()], false, None);
    ///
    /// // Load a directory recursively with image filter
    /// let filter = vec!["jpg".to_string(), "png".to_string()];
    /// let images = FileManager::load_files(vec![dir.to_path_buf()], true, Some(filter));
    /// ```
    pub fn load_files(
        paths: Vec<PathBuf>,
        recursive: bool,
        extensions_filter: Option<Vec<String>>,
    ) -> Result<Vec<FileItem>> {
        // Validate that at least one path was provided
        if paths.is_empty() {
            anyhow::bail!("No paths provided for file loading");
        }

        let mut file_items = Vec::new();

        for path in paths {
            match Self::process_path(&path, recursive, &extensions_filter) {
                Ok(mut items) => file_items.append(&mut items),
                Err(e) => {
                    log::warn!("Failed to process path {:?}: {}", path, e);
                    // Continue processing other paths instead of failing entirely
                    // This allows partial success when some paths are invalid
                }
            }
        }

        // Validate that we found at least one file
        if file_items.is_empty() {
            anyhow::bail!("No valid files found in provided paths");
        }

        Ok(file_items)
    }

    /// Process a single path (file or directory)
    fn process_path(
        path: &PathBuf,
        recursive: bool,
        extensions_filter: &Option<Vec<String>>,
    ) -> Result<Vec<FileItem>> {
        let mut file_items = Vec::new();

        if path.is_file() {
            // Single file - create FileItem directly
            let item = Self::create_file_item(path)?;
            file_items.push(item);
        } else if path.is_dir() {
            // Directory - scan contents
            if recursive {
                // Recursive mode: use WalkDir for deep traversal
                // Note: We do NOT use follow_links(true) to avoid infinite loops
                // from circular symlinks/reparse points (common on Windows).
                // Max depth is also capped for safety.
                let walker = WalkDir::new(path)
                    .max_depth(100)
                    .into_iter()
                    .filter_map(|e| {
                        if let Err(err) = &e {
                            log::warn!("WalkDir error (skipping): {}", err);
                        }
                        e.ok()
                    })
                    .filter(|e| e.file_type().is_file());

                for entry in walker {
                    // Apply extension filter if provided
                    if let Some(ref filter) = extensions_filter {
                        if !Self::matches_extension_filter(entry.path(), filter) {
                            continue;
                        }
                    }

                    match Self::create_file_item(entry.path()) {
                        Ok(item) => file_items.push(item),
                        Err(e) => {
                            log::warn!("Skipping file {:?} due to error: {}", entry.path(), e);
                        }
                    }
                }
            } else {
                // Non-recursive mode: only scan top-level files using fs::read_dir
                let entries = fs::read_dir(path)?;

                for entry_result in entries {
                    match entry_result {
                        Ok(entry) => {
                            let entry_path = entry.path();

                            // Skip directories in non-recursive mode
                            if !entry_path.is_file() {
                                continue;
                            }

                            // Apply extension filter if provided
                            if let Some(ref filter) = extensions_filter {
                                if !Self::matches_extension_filter(&entry_path, filter) {
                                    continue;
                                }
                            }

                            match Self::create_file_item(&entry_path) {
                                Ok(item) => file_items.push(item),
                                Err(e) => {
                                    log::warn!(
                                        "Skipping file {:?} due to error: {}",
                                        entry_path,
                                        e
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            log::warn!("Failed to read directory entry: {}", e);
                        }
                    }
                }
            }
        } else {
            // Path doesn't exist or is not a regular file/directory
            anyhow::bail!(
                "Invalid path: {:?}. Path does not exist or is not accessible.",
                path
            );
        }

        Ok(file_items)
    }

    /// Check if a file's extension matches the filter criteria
    ///
    /// # Arguments
    /// * `file_path` - Path to the file being checked
    /// * `filter` - List of allowed extensions (lowercase, without dots)
    ///
    /// # Returns
    /// true if the file matches the filter (or has no extension when filtering), false otherwise
    fn matches_extension_filter(file_path: &std::path::Path, filter: &[String]) -> bool {
        // Get the file extension
        let ext = match file_path.extension() {
            Some(e) => e.to_string_lossy().to_lowercase(),
            None => {
                // Files without extension don't match any extension filter
                return false;
            }
        };

        // Check if this extension is in our allowed list
        filter
            .iter()
            .any(|allowed_ext| allowed_ext.to_lowercase() == ext)
    }

    /// Create a single FileItem from a filesystem path
    /// Extracts metadata (size, timestamps) and creates the data structure
    ///
    /// # Arguments
    /// * `path` - Absolute path to the file
    ///
    /// # Returns
    /// A `Result` containing the created `FileItem`, or an error if:
    /// - The path doesn't exist
    /// - Metadata cannot be read
    /// - Filename cannot be parsed
    fn create_file_item(path: &std::path::Path) -> Result<FileItem> {
        // Validate that the path exists
        if !path.exists() {
            anyhow::bail!("Path does not exist: {:?}", path);
        }

        // Read filesystem metadata (size, timestamps, attributes)
        let metadata = fs::metadata(path)
            .with_context(|| format!("Failed to read filesystem metadata for {:?}", path))?;

        // Extract filename components
        let name_stem = path
            .file_stem()
            .ok_or_else(|| {
                anyhow::anyhow!("Invalid filename: cannot extract name stem from {:?}", path)
            })?
            .to_string_lossy()
            .to_string();

        let extension = path
            .extension()
            .map(|e| format!(".{}", e.to_string_lossy()))
            .unwrap_or_default();

        // Convert filesystem times to DateTime<Utc> for consistent handling
        let created_time = metadata
            .created()
            .map(Into::into)
            .unwrap_or_else(|_| chrono::Utc::now());

        let modified_time = metadata
            .modified()
            .map(Into::into)
            .unwrap_or_else(|_| chrono::Utc::now());

        let accessed_time = metadata
            .accessed()
            .map(Into::into)
            .unwrap_or_else(|_| chrono::Utc::now());

        // Extract extended metadata (EXIF/ID3) for supported file types
        let file_metadata =
            if MetadataReader::is_supported_extension(&extension.trim_start_matches('.')) {
                Some(MetadataReader::extract(path))
            } else {
                None
            };

        Ok(FileItem {
            id: uuid::Uuid::new_v4(),
            original_path: path.to_path_buf(),
            original_name: name_stem,
            original_ext: extension,
            preview_name: None,
            file_size: metadata.len(),
            created_time,
            modified_time,
            accessed_time,
            metadata: file_metadata,
            status: FileStatus::Pending,
        })
    }
}
