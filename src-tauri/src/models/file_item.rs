use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::FileMetadata;

/// Processing status of a file during batch operations
/// Tracks the lifecycle of each file through the rename pipeline
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[derive(Default)]
pub enum FileStatus {
    /// Awaiting processing (initial state)
    #[default]
    Pending,

    /// Preview has been generated successfully
    PreviewReady,

    /// Currently being processed by the rename engine
    Processing,

    /// Successfully renamed
    Success,

    /// Failed with error message explaining the failure reason
    Failed(String),

    /// Name conflict detected (target name already exists)
    Conflict,

    /// Skipped (user choice or error policy configuration)
    Skipped,
}


impl std::fmt::Display for FileStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileStatus::Pending => write!(f, "Pending"),
            FileStatus::PreviewReady => write!(f, "Preview Ready"),
            FileStatus::Processing => write!(f, "Processing"),
            FileStatus::Success => write!(f, "Success"),
            FileStatus::Failed(msg) => write!(f, "Failed: {}", msg),
            FileStatus::Conflict => write!(f, "Conflict"),
            FileStatus::Skipped => write!(f, "Skipped"),
        }
    }
}

/// Represents a single file in the rename operation
/// This is the core data structure that flows through the entire pipeline
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileItem {
    /// Unique identifier for this file entry (UUID v4)
    pub id: Uuid,

    /// Original full path to the file (absolute path)
    pub original_path: std::path::PathBuf,

    /// Original filename without extension
    /// Example: "photo" for "photo.jpg"
    pub original_name: String,

    /// Original extension including dot (e.g., ".jpg", ".txt")
    /// Empty string for files without extension
    pub original_ext: String,

    /// Preview of new name after applying methods (computed by preview engine)
    /// None means preview has not been calculated yet
    pub preview_name: Option<String>,

    /// File size in bytes
    pub file_size: u64,

    /// File creation time (from filesystem metadata)
    pub created_time: DateTime<Utc>,

    /// File modification time (last write time)
    pub modified_time: DateTime<Utc>,

    /// File last access time
    pub accessed_time: DateTime<Utc>,

    /// Extended metadata (EXIF for images, ID3 for audio, etc.)
    /// None if not yet extracted or not applicable
    pub metadata: Option<FileMetadata>,

    /// Current processing status in the batch operation
    pub status: FileStatus,
}

impl FileItem {
    /// Create a new FileItem from path components
    ///
    /// # Arguments
    /// * `path` - Full absolute path to the file
    /// * `name` - Filename stem (without extension)
    /// * `ext` - Extension including dot (e.g., ".jpg")
    ///
    /// # Returns
    /// A new FileItem with default values and Pending status
    pub fn new(path: std::path::PathBuf, name: String, ext: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            original_path: path,
            original_name: name,
            original_ext: ext,
            preview_name: None,
            file_size: 0,
            created_time: Utc::now(),
            modified_time: Utc::now(),
            accessed_time: Utc::now(),
            metadata: None,
            status: FileStatus::Pending,
        }
    }

    /// Get the full original filename (name + extension)
    ///
    /// # Examples
    /// ```
    /// // For "photo.jpg": returns "photo.jpg"
    /// // For "README": returns "README" (no extension)
    /// ```
    pub fn full_original_name(&self) -> String {
        format!("{}{}", self.original_name, self.original_ext)
    }

    /// Check if this file item has been successfully processed
    pub fn is_success(&self) -> bool {
        matches!(self.status, FileStatus::Success)
    }

    /// Check if this file item has an error or conflict
    pub fn has_error(&self) -> bool {
        matches!(self.status, FileStatus::Failed(_) | FileStatus::Conflict)
    }
}
