use anyhow::Result;

use crate::models::{ApplyToOption, FileMetadata, MethodConfig, MethodType};

/// Context information available during method execution
/// Provides access to file metadata, position in batch, and other runtime data
#[derive(Debug, Clone)]
pub struct MethodContext {
    /// Index of current file in the batch (0-based)
    /// Used for numbering, sequencing, and relative positioning
    pub file_index: usize,
    
    /// Total number of files in the current batch operation
    /// Useful for calculating progress percentages or padding
    pub total_files: usize,
    
    /// Metadata for the current file (if available and extracted)
    /// Contains EXIF/ID3 data when applicable
    pub file_metadata: Option<FileMetadata>,
    
    /// Original filename without extension
    /// Available for reference during transformations
    pub original_name: String,
    
    /// Original extension including dot (e.g., ".jpg")
    /// Available for reference during transformations
    pub original_ext: String,
    
    /// File created timestamp (ISO 8601 format, e.g., "2024-01-15T10:30:00")
    /// Used by Timestamp method
    pub created_time: Option<String>,
    
    /// File last modified timestamp (ISO 8601 format)
    /// Used by Timestamp method
    pub modified_time: Option<String>,
    
    /// File last accessed timestamp (ISO 8601 format)
    /// Used by Timestamp method
    pub accessed_time: Option<String>,
}

impl Default for MethodContext {
    fn default() -> Self {
        Self {
            file_index: 0,
            total_files: 1,
            file_metadata: None,
            original_name: String::new(),
            original_ext: String::new(),
            created_time: None,
            modified_time: None,
            accessed_time: None,
        }
    }
}

/// Core trait that all renaming methods must implement
/// This trait defines the interface for the Strategy pattern used by the Pipeline executor
pub trait Method: Send + Sync + std::fmt::Debug {
    /// Human-readable name of this method (e.g., "Replace", "Add")
    /// Used for display in UI, logging, and error messages
    fn name(&self) -> &str;
    
    /// Type identifier for serialization and method routing
    /// Must match one of the variants in `MethodType` enum
    fn method_type(&self) -> MethodType;
    
    /// Apply this method's transformation to the input string
    /// 
    /// # Arguments
    /// * `input` - The current filename (or portion thereof) to transform
    /// * `context` - Execution context with file info, metadata, etc.
    /// 
    /// # Returns
    /// The transformed string on success, or an error if transformation fails.
    /// 
    /// # Errors
    /// Returns error if:
    /// - Input is invalid for this method's logic
    /// - Configuration is invalid (should be caught by validate() first)
    /// - Runtime error occurs during transformation
    fn apply(&self, input: &str, context: &MethodContext) -> Result<String>;
    
    /// Validate the method's configuration is valid before use
    /// Called before adding to pipeline to catch configuration errors early
    /// 
    /// # Returns
    /// Ok(()) if configuration is valid, Err with descriptive message otherwise
    fn validate(&self) -> Result<()>;
    
    /// Serialize this method to a config struct for persistence
    /// Allows saving/loading method configurations to files
    fn to_config(&self) -> MethodConfig;
    
    /// Get which part(s) of the filename this method applies to
    /// Determines whether the method affects Name, Extension, or Both
    fn apply_to(&self) -> ApplyToOption;
}
