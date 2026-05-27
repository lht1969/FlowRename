use serde::{Deserialize, Serialize};
use crate::method_engine::sanitize_filename;
use tauri::State;
use std::fs;
use std::path::Path;

use crate::commands::file_commands::AppState;

/// Undo history entry stored in AppState
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UndoEntry {
    /// Unique identifier for this undo entry
    pub id: String,
    
    /// List of rename operations that can be reversed
    pub operations: Vec<UndoOperation>,
    
    /// Timestamp when the undo entry was created
    pub created_at: String,
}

/// Single reversible rename operation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UndoOperation {
    /// Path after rename (current location)
    pub current_path: String,
    
    /// Original name before rename
    pub original_name: String,
}

/// Response structure for undo operation
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UndoResponse {
    /// Whether the undo was successful
    pub success: bool,
    
    /// Number of files restored
    pub restored_count: usize,
    
    /// Error message if undo failed
    pub error: Option<String>,
}

/// Request structure for executing rename operations
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteRenameRequest {
    /// List of files to rename with their target names
    pub rename_items: Vec<RenameItem>,
    
    /// Whether to create undo history before renaming
    #[serde(default = "default_undo_enabled")]
    pub create_undo_history: bool,
}

fn default_undo_enabled() -> bool { true }

/// Single item in a batch rename operation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameItem {
    /// Current file path
    pub current_path: String,
    
    /// Target new name (not full path, just filename)
    pub new_name: String,
}

/// Response structure for rename execution results
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteResponse {
    /// Number of files successfully renamed
    pub success_count: usize,
    
    /// Number of files that failed to rename
    pub failed_count: usize,
    
    /// List of failed renames with error details
    pub errors: Vec<RenameError>,
    
    /// Processing time in milliseconds
    pub elapsed_ms: u64,
    
    /// Error message if critical failure occurred
    pub error: Option<String>,
}

/// Details about a failed rename operation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameError {
    /// Original file path that failed
    pub original_path: String,
    
    /// Intended new name
    pub attempted_name: String,
    
    /// Reason for failure
    pub reason: String,
}

/// Command: Execute the actual file rename operations
/// 
/// This command performs the actual filesystem operations to rename files.
/// It should only be called after user confirms the preview results.
/// 
/// # Safety Features:
/// - Validates all paths exist before starting
/// - Performs renames sequentially to avoid race conditions
/// - Creates undo history if requested
/// - Returns detailed error information for each failure
/// 
/// # Frontend Usage
/// ```typescript
/// const result = await invoke<ExecuteResponse>('execute_rename', {
///   request: {
///     renameItems: [
///       { currentPath: 'C:/Photos/old_name.jpg', newName: 'new_name.jpg' }
///     ],
///     createUndoHistory: true
///   }
/// });
/// ```
#[tauri::command]
pub async fn execute_rename(
    state: State<'_, AppState>,
    request: ExecuteRenameRequest,
) -> Result<ExecuteResponse, String> {
    let start_time = std::time::Instant::now();
    
    log::info!(
        "Executing {} rename operations (undo={})",
        request.rename_items.len(),
        request.create_undo_history
    );
    
    if request.rename_items.is_empty() {
        return Ok(ExecuteResponse {
            success_count: 0,
            failed_count: 0,
            errors: vec![],
            elapsed_ms: start_time.elapsed().as_millis() as u64,
            error: Some("No items to rename".to_string()),
        });
    }
    
    // TODO: Create undo history snapshot if requested
    // This will be implemented in Task 9 (Undo Manager)
    if request.create_undo_history {
        log::info!("Undo history creation requested (pending Task 9 implementation)");
    }
    
    let mut success_count = 0;
    let mut failed_count = 0;
    let mut errors: Vec<RenameError> = Vec::new();
    let mut undo_ops: Vec<UndoOperation> = Vec::new();
    
    // Process each rename item
    for item in &request.rename_items {
        let source_path = Path::new(&item.current_path);
        
        // Validate source exists
        if !source_path.exists() {
            failed_count += 1;
            errors.push(RenameError {
                original_path: item.current_path.clone(),
                attempted_name: item.new_name.clone(),
                reason: "Source file does not exist".to_string(),
            });
            continue;
        }
        
        // Build target path (same directory, new filename)
        if let Some(parent) = source_path.parent() {
            // 清理文件名中的非法字符（二次安全防护）
            let safe_name = sanitize_filename(&item.new_name);
            let target_path = parent.join(&safe_name);
            
            // Check if target already exists
            if target_path.exists() && target_path != source_path {
                failed_count += 1;
                errors.push(RenameError {
                    original_path: item.current_path.clone(),
                    attempted_name: item.new_name.clone(),
                    reason: format!("Target already exists: {}", target_path.display()),
                });
                continue;
            }
            
            // Perform the actual rename
            match fs::rename(&source_path, &target_path) {
                Ok(_) => {
                    success_count += 1;
                    // Record undo operation
                    undo_ops.push(UndoOperation {
                        current_path: target_path.to_string_lossy().to_string(),
                        original_name: source_path.file_name()
                            .map(|f| f.to_string_lossy().to_string())
                            .unwrap_or_default(),
                    });
                    log::debug!(
                        "Renamed '{}' -> '{}'",
                        source_path.display(),
                        target_path.display()
                    );
                }
                Err(e) => {
                    failed_count += 1;
                    errors.push(RenameError {
                        original_path: item.current_path.clone(),
                        attempted_name: item.new_name.clone(),
                        reason: format!("Filesystem error: {}", e),
                    });
                }
            }
        } else {
            failed_count += 1;
            errors.push(RenameError {
                original_path: item.current_path.clone(),
                attempted_name: item.new_name.clone(),
                reason: "Could not determine parent directory".to_string(),
            });
        }
    }
    
    let elapsed_ms = start_time.elapsed().as_millis() as u64;
    
    log::info!(
        "Rename completed: {} succeeded, {} failed in {}ms",
        success_count,
        failed_count,
        elapsed_ms
    );
    
    // Clear file list after successful rename (forces rescan)
    if failed_count == 0 {
        match state.files.lock() {
            Ok(mut guard) => guard.clear(),
            Err(e) => log::warn!("Failed to clear file list: {}", e),
        }
    }
    
    // Save undo history if operations were successful and requested
    if request.create_undo_history && !undo_ops.is_empty() {
        let entry = UndoEntry {
            id: format!("undo_{}", chrono::Utc::now().timestamp_millis()),
            operations: undo_ops,
            created_at: chrono::Utc::now().to_rfc3339(),
        };
        match state.undo_history.lock() {
            Ok(mut history) => {
                // Use UndoManager to persist the entry to disk
                if let Err(e) = state.undo_manager.add_entry(&mut history, entry) {
                    log::warn!("Failed to persist undo history to disk: {}", e);
                } else {
                    log::info!("Undo history saved with {} operations", history.last().map(|e| e.operations.len()).unwrap_or(0));
                }
            }
            Err(e) => log::warn!("Failed to save undo history: {}", e),
        }
    }
    
    Ok(ExecuteResponse {
        success_count,
        failed_count,
        errors,
        elapsed_ms,
        error: None,
    })
}

/// Command: Undo the last rename operation
/// 
/// Reverses the most recent batch rename by restoring files to their
/// original names. Only one level of undo is supported in this implementation.
/// 
/// # Frontend Usage
/// ```typescript
/// const result = await invoke<UndoResponse>('undo_last_rename');
/// ```
#[tauri::command]
pub async fn undo_last_rename(
    state: State<'_, AppState>,
) -> Result<UndoResponse, String> {
    log::info!("Undo last rename requested");
    
    // Pop the last undo entry using UndoManager for persistence
    let entry = {
        match state.undo_history.lock() {
            Ok(mut history) => {
                if history.is_empty() {
                    return Ok(UndoResponse {
                        success: false,
                        restored_count: 0,
                        error: Some("No undo history available".to_string()),
                    });
                }
                // Use UndoManager to pop and persist the change
                state.undo_manager.pop_entry(&mut history)
            }
            Err(e) => {
                return Ok(UndoResponse {
                    success: false,
                    restored_count: 0,
                    error: Some(format!("Failed to access undo history: {}", e)),
                });
            }
        }
    };
    
    let Some(entry) = entry else {
        return Ok(UndoResponse {
            success: false,
            restored_count: 0,
            error: Some("No undo entry found".to_string()),
        });
    };
    
    let mut restored_count = 0;
    
    // Reverse each operation
    for op in &entry.operations {
        let current_path = Path::new(&op.current_path);
        
        if !current_path.exists() {
            log::warn!("Undo skip: file not found at {}", op.current_path);
            continue;
        }
        
        if let Some(parent) = current_path.parent() {
            let original_path = parent.join(&op.original_name);
            
            match fs::rename(current_path, &original_path) {
                Ok(_) => {
                    restored_count += 1;
                    log::debug!("Undo: '{}' -> '{}'", current_path.display(), original_path.display());
                }
                Err(e) => {
                    log::error!("Undo failed for '{}': {}", current_path.display(), e);
                }
            }
        }
    }
    
    log::info!("Undo completed: {} files restored", restored_count);
    
    Ok(UndoResponse {
        success: restored_count > 0,
        restored_count,
        error: None,
    })
}

/// Response structure for undo status query
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UndoStatusResponse {
    /// Whether there is undo history available
    pub has_history: bool,
    
    /// Number of undo entries available
    pub entry_count: usize,
    
    /// Total number of reversible operations across all entries
    pub total_operations: usize,
    
    /// Timestamp of the most recent undo entry
    pub last_entry_time: Option<String>,
}

/// Command: Query the current undo history status
/// 
/// Returns information about available undo entries without modifying them.
/// Useful for enabling/disabling the undo button in the UI.
#[tauri::command]
pub async fn get_undo_status(
    state: State<'_, AppState>,
) -> Result<UndoStatusResponse, String> {
    match state.undo_history.lock() {
        Ok(history) => {
            let entry_count = history.len();
            let total_operations = history.iter().map(|e| e.operations.len()).sum();
            let last_entry_time = history.last().map(|e| e.created_at.clone());

            Ok(UndoStatusResponse {
                has_history: entry_count > 0,
                entry_count,
                total_operations,
                last_entry_time,
            })
        }
        Err(e) => Err(format!("Failed to access undo history: {}", e)),
    }
}
