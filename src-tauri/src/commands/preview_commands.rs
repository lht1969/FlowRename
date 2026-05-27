use serde::{Deserialize, Serialize};
use tauri::State;

use crate::commands::file_commands::AppState;
use crate::method_engine::{Pipeline, MethodContext, sanitize_filename};
use crate::models::method_config::MethodConfig;

/// Request structure for previewing rename operations
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewRequest {
    /// List of method configurations to apply in sequence
    pub methods: Vec<MethodConfig>,
    
    /// Optional: Custom template for NewName method (e.g., "<Date>_<Inc:3>_<Name>")
    #[serde(default)]
    pub template: Option<String>,
}

/// Response structure for rename preview results
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewResponse {
    /// List of file items with their new names applied
    pub files: Vec<FilePreviewItem>,
    
    /// Total number of files processed
    pub total_count: usize,
    
    /// Number of files that would be renamed (name changed)
    pub changed_count: usize,
    
    /// Number of conflicts detected (duplicate target names)
    pub conflict_count: usize,
    
    /// Processing time in milliseconds
    pub elapsed_ms: u64,
    
    /// Error message if processing failed
    pub error: Option<String>,
}

/// Single file's preview result with original and new name
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilePreviewItem {
    /// Original file path
    pub original_path: String,
    
    /// Original filename
    pub original_name: String,
    
    /// New filename after applying all methods
    pub new_name: String,
    
    /// Whether the name actually changed
    pub is_changed: bool,
    
    /// Conflict flag - true if another file has the same target name
    pub has_conflict: bool,
}

/// Command: Preview rename operations without executing them
/// 
/// This command takes the current file list and applies all configured renaming
/// methods to generate a preview of what the final result would look like.
/// It detects potential naming conflicts before actual execution.
/// 
/// # Frontend Usage
/// ```typescript
/// const preview = await invoke<PreviewResponse>('preview_rename', {
///   request: {
///     methods: [
///       { type: 'Add', config: { text: 'IMG_', position: 'Start' } },
///       { type: 'NewCase', config: { newCase: 'Upper' } }
///     ],
///     template: '<Date:YYYY>-<Inc:3>_<Name>'
///   }
/// });
/// ```
#[tauri::command]
pub async fn preview_rename(
    state: State<'_, AppState>,
    request: PreviewRequest,
) -> Result<PreviewResponse, String> {
    let start_time = std::time::Instant::now();

    log::info!(
        "Previewing rename with {} methods, template={:?}",
        request.methods.len(),
        request.template
    );

    // Debug: log the first method config as JSON to see what frontend sent
    if let Some(first) = request.methods.first() {
        match serde_json::to_string(first) {
            Ok(json) => log::info!("First method config JSON: {}", json),
            Err(e) => log::warn!("Failed to serialize first method: {}", e),
        }
    }
    
    // Get current file list from state
    let files = match state.files.lock() {
        Ok(guard) => guard.clone(),
        Err(e) => return Err(format!("Failed to access file list: {}", e)),
    };
    
    if files.is_empty() {
        return Ok(PreviewResponse {
            files: vec![],
            total_count: 0,
            changed_count: 0,
            conflict_count: 0,
            elapsed_ms: start_time.elapsed().as_millis() as u64,
            error: Some("No files loaded. Please scan a directory first.".to_string()),
        });
    }
    
    // Build pipeline from method configurations
    let mut pipeline = Pipeline::new();
    for method_config in &request.methods {
        match crate::methods::create_method_from_config(method_config) {
            Ok(method) => pipeline.add_method(method),
            Err(e) => {
                return Ok(PreviewResponse {
                    files: vec![],
                    total_count: 0,
                    changed_count: 0,
                    conflict_count: 0,
                    elapsed_ms: start_time.elapsed().as_millis() as u64,
                    error: Some(format!("Invalid method configuration: {}", e)),
                });
            }
        }
    }
    
    let mut preview_items: Vec<FilePreviewItem> = Vec::new();
    let mut changed_count = 0;
    let total_files = files.len();
    
    // Process each file through the pipeline
    for (index, file) in files.iter().enumerate() {
        // Build execution context for this file
        let context = MethodContext {
            file_index: index,
            total_files,
            file_metadata: file.metadata.clone(),
            original_name: file.original_name.clone(),
            original_ext: file.original_ext.clone(),
            created_time: Some(file.created_time.to_rfc3339()),
            modified_time: Some(file.modified_time.to_rfc3339()),
            accessed_time: Some(file.accessed_time.to_rfc3339()),
        };

        // For ApplyToExtension case, pipeline needs the original full name
        // For ApplyToName case, pipeline gets template result (stem) but with ext context
        let full_original = format!("{}{}", file.original_name, file.original_ext);

        // Pipeline receives full original name so it can apply methods to name/ext/both appropriately
        // Pipeline 执行后清理文件名中的非法字符
        let new_name = match pipeline.execute(&full_original, &context) {
            Ok(name) => sanitize_filename(&name),
            Err(_) => full_original.clone(),
        };

        let is_changed = new_name != full_original;
        if is_changed {
            changed_count += 1;
        }

        preview_items.push(FilePreviewItem {
            original_path: file.original_path.display().to_string(),
            original_name: full_original,
            new_name,
            is_changed,
            has_conflict: false,
        });
    }
    
    // Detect naming conflicts (duplicate target names)
    let mut conflict_count = 0;
    let mut seen_names: std::collections::HashSet<String> = std::collections::HashSet::new();
    
    for item in &mut preview_items {
        if item.is_changed && !seen_names.insert(item.new_name.clone()) {
            item.has_conflict = true;
            conflict_count += 1;
        }
    }
    
    // Mark first occurrence as non-conflict (only duplicates are conflicts)
    if conflict_count > 0 {
        let mut first_occurrences: std::collections::HashMap<String, bool> = std::collections::HashMap::new();
        for item in &mut preview_items {
            if item.has_conflict {
                let entry = first_occurrences.entry(item.new_name.clone()).or_insert(false);
                if !*entry {
                    item.has_conflict = false;  // First occurrence is OK
                    *entry = true;
                    conflict_count -= 1;
                }
            }
        }
    }
    
    let elapsed_ms = start_time.elapsed().as_millis() as u64;
    
    log::info!(
        "Preview completed: {} files processed, {} changed, {} conflicts in {}ms",
        total_files,
        changed_count,
        conflict_count,
        elapsed_ms
    );
    
    Ok(PreviewResponse {
        files: preview_items,
        total_count: total_files,
        changed_count,
        conflict_count,
        elapsed_ms,
        error: None,
    })
}

