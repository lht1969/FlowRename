// File Manager module - handles file I/O, scanning, and metadata operations
// This module provides the interface between the application and the filesystem

pub mod scanner;

// Re-export the main File Manager struct for convenient access
pub use scanner::FileManager;
