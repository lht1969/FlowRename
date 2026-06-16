use serde::{Deserialize, Serialize};
use tauri::State;
use std::sync::Mutex;
use std::path::PathBuf;

use crate::file_manager::scanner::{FileManager, sort_file_items};
use crate::models::file_item::FileItem;
use crate::commands::rename_commands::UndoEntry;
use crate::undo_manager::UndoManager;

/// Application state shared across all Tauri commands
pub struct AppState {
    /// Current list of loaded files (for rename operations)
    pub files: Mutex<Vec<FileItem>>,
    
    /// Currently selected directory path
    pub current_dir: Mutex<Option<String>>,
    
    /// Undo history stack for rename operations
    pub undo_history: Mutex<Vec<UndoEntry>>,
    
    /// Persistent undo manager for saving/loading history to disk
    pub undo_manager: UndoManager,
}

/// Request structure for scanning files
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanRequest {
    /// Directory path to scan
    pub directory_path: String,
    
    /// Whether to scan recursively into subdirectories
    #[serde(default = "default_recursive")]
    pub recursive: bool,
    
    /// File extensions to filter (empty = all files)
    #[serde(default = "default_extensions")]
    pub file_extensions: Vec<String>,
}

fn default_recursive() -> bool { false }
fn default_extensions() -> Vec<String> { vec![] }

/// Response structure for file scanning results
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanResponse {
    /// List of files found in the directory
    pub files: Vec<FileItem>,
    
    /// Total number of files found
    pub total_count: usize,
    
    /// Number of directories scanned
    pub directories_scanned: usize,
    
    /// Time taken for the operation (milliseconds)
    pub elapsed_ms: u64,
    
    /// Error message if any issues occurred
    pub error: Option<String>,
}

/// Command: Scan a directory and return list of files
/// 
/// This command triggers the file scanner to load files from the specified directory.
/// Results are stored in app state and returned to the frontend for display.
/// 
/// # Frontend Usage
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
/// const result = await invoke<ScanResponse>('scan_directory', {
///   request: {
///     directoryPath: 'C:/Users/Photos',
///     recursive: true,
///     fileExtensions: ['jpg', 'png']
///   }
/// });
/// ```
#[tauri::command]
pub async fn scan_directory(
    state: State<'_, AppState>,
    request: ScanRequest,
) -> Result<ScanResponse, String> {
    let start_time = std::time::Instant::now();
    
    log::info!(
        "Scanning directory: {} (recursive={}, extensions={:?})",
        request.directory_path,
        request.recursive,
        request.file_extensions
    );
    
    // Convert request to FileManager API format
    let paths = vec![PathBuf::from(&request.directory_path)];
    let extensions_filter = if request.file_extensions.is_empty() {
        None
    } else {
        Some(request.file_extensions.clone())
    };
    
    // Execute file scan using FileManager
    match FileManager::load_files(paths, request.recursive, extensions_filter) {
        Ok(files) => {
            let total_count = files.len();
            
            // Store files in application state for later use
            match state.files.lock() {
                Ok(mut guard) => *guard = files.clone(),
                Err(e) => return Err(format!("Failed to lock app state: {}", e)),
            }
            
            // Store current directory
            match state.current_dir.lock() {
                Ok(mut guard) => *guard = Some(request.directory_path),
                Err(e) => log::warn!("Failed to store current dir: {}", e),
            }
            
            let elapsed_ms = start_time.elapsed().as_millis() as u64;
            
            log::info!("Scan completed: {} files found in {}ms", total_count, elapsed_ms);
            
            Ok(ScanResponse {
                files,
                total_count,
                directories_scanned: 1,  // Simplified - could track actual count
                elapsed_ms,
                error: None,
            })
        }
        Err(e) => {
            let error_msg = format!("Directory scan failed: {}", e);
            log::error!("{}", error_msg);
            
            Ok(ScanResponse {
                files: vec![],
                total_count: 0,
                directories_scanned: 0,
                elapsed_ms: start_time.elapsed().as_millis() as u64,
                error: Some(error_msg),
            })
        }
    }
}

/// Request structure for sorting files
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SortFilesRequest {
    /// List of files to sort (passed by value since sorting is in-place)
    pub files: Vec<FileItem>,

    /// Field to sort by: "name", "size", or "modified"
    #[serde(default = "default_sort_field")]
    pub field: String,

    /// Sort in descending order (true) or ascending order (false)
    #[serde(default = "default_sort_desc")]
    pub desc: bool,
}

fn default_sort_field() -> String {
    "name".to_string()
}

fn default_sort_desc() -> bool {
    false
}

/// Response structure for sort results
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SortFilesResponse {
    pub files: Vec<FileItem>,
}

/// Command: Sort a list of files by the specified field and direction.
///
/// Uses natural sorting (StrCmpLogicalW style) for filename comparisons to match
/// Windows Explorer's sorting behavior.
#[tauri::command]
pub async fn sort_files(request: SortFilesRequest) -> Result<SortFilesResponse, String> {
    let mut files = request.files;
    sort_file_items(&mut files, &request.field, request.desc);
    Ok(SortFilesResponse { files })
}

/// Command: Clear the current file list from memory
/// 
/// Frees up memory by clearing the stored file list when user navigates away
/// or starts a new session.
#[tauri::command]
pub async fn clear_files(state: State<'_, AppState>) -> Result<(), String> {
    match state.files.lock() {
        Ok(mut guard) => guard.clear(),
        Err(e) => return Err(format!("Failed to clear files: {}", e)),
    }
    
    if let Ok(mut guard) = state.current_dir.lock() { *guard = None }
    
    log::info!("File list cleared");
    Ok(())
}

/// Request structure for scanning specific files
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanFilesRequest {
    /// List of file paths to load
    pub file_paths: Vec<String>,
}

/// Command: Scan specific files and return list of files
///
/// This command loads metadata for a list of user-selected file paths.
/// Results are stored in app state and returned to the frontend for display.
#[tauri::command]
pub async fn scan_files(
    state: State<'_, AppState>,
    request: ScanFilesRequest,
) -> Result<ScanResponse, String> {
    let start_time = std::time::Instant::now();

    log::info!("Scanning {} specific files", request.file_paths.len());

    // Convert string paths to PathBuf
    let paths: Vec<PathBuf> = request.file_paths.iter().map(PathBuf::from).collect();

    // Execute file scan using FileManager (non-recursive, no extension filter)
    match FileManager::load_files(paths, false, None) {
        Ok(files) => {
            let total_count = files.len();

            // Store files in application state for later use
            match state.files.lock() {
                Ok(mut guard) => *guard = files.clone(),
                Err(e) => return Err(format!("Failed to lock app state: {}", e)),
            }

            // Clear current directory since we're loading specific files
            match state.current_dir.lock() {
                Ok(mut guard) => *guard = None,
                Err(e) => log::warn!("Failed to clear current dir: {}", e),
            }

            let elapsed_ms = start_time.elapsed().as_millis() as u64;

            log::info!("File scan completed: {} files loaded in {}ms", total_count, elapsed_ms);

            Ok(ScanResponse {
                files,
                total_count,
                directories_scanned: 0,
                elapsed_ms,
                error: None,
            })
        }
        Err(e) => {
            let error_msg = format!("File scan failed: {}", e);
            log::error!("{}", error_msg);

            Ok(ScanResponse {
                files: vec![],
                total_count: 0,
                directories_scanned: 0,
                elapsed_ms: start_time.elapsed().as_millis() as u64,
                error: Some(error_msg),
            })
        }
    }
}
