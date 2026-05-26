// Models module - Core data structures for AdRename
// This module defines all data types used throughout the application

pub mod file_item;
pub mod metadata;
pub mod method_config;

// Re-export commonly used types for convenient imports
pub use file_item::{FileItem, FileStatus};
pub use metadata::{FileMetadata, ImageMetadata, AudioMetadata};
pub use method_config::{
    ApplyToOption, MethodConfig, MethodType, ReplaceConfig, AddConfig, RemoveConfig,
    NewCaseConfig, NewNameConfig, OccurrenceOption, AddPosition,
    RemovePosition, CaseType, CaseLocation,
    ListConfig, ListOverflow, MoveConfig, TrimConfig,
    RenumberConfig, RenumberPosition, TimestampConfig, TimestampSource,
};
