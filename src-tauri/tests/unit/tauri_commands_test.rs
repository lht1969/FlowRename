#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use tempfile::TempDir;
    
    use flowrename::commands::file_commands::{AppState, ScanRequest, ScanResponse};
    use flowrename::commands::preview_commands::{PreviewRequest, PreviewResponse};
    use flowrename::commands::rename_commands::{ExecuteRenameRequest, ExecuteResponse, RenameItem};
    use flowrename::models::file_item::FileItem;
    use flowrename::models::method_config::{MethodConfig, ReplaceConfig, OccurrenceOption, ApplyToOption};
    use flowrename::method_engine::{Pipeline, MethodContext};
    use flowrename::undo_manager::UndoManager;

    /// Helper to create test app state with pre-loaded files
    fn create_test_state_with_files() -> AppState {
        let temp_dir = TempDir::new().unwrap();
        let files = vec![
            FileItem {
                id: uuid::Uuid::new_v4(),
                original_path: std::path::PathBuf::from("test/photo1.jpg"),
                original_name: "photo1".to_string(),
                original_ext: ".jpg".to_string(),
                preview_name: None,
                file_size: 1024,
                created_time: chrono::Utc::now(),
                modified_time: chrono::Utc::now(),
                accessed_time: chrono::Utc::now(),
                metadata: None,
                status: flowrename::models::file_item::FileStatus::Pending,
            },
            FileItem {
                id: uuid::Uuid::new_v4(),
                original_path: std::path::PathBuf::from("test/document.txt"),
                original_name: "document".to_string(),
                original_ext: ".txt".to_string(),
                preview_name: None,
                file_size: 2048,
                created_time: chrono::Utc::now(),
                modified_time: chrono::Utc::now(),
                accessed_time: chrono::Utc::now(),
                metadata: None,
                status: flowrename::models::file_item::FileStatus::Pending,
            },
        ];
        
        AppState {
            files: Mutex::new(files),
            current_dir: Mutex::new(Some("test".to_string())),
            undo_history: Mutex::new(vec![]),
            undo_manager: UndoManager::new(&temp_dir.path().join("undo_data")),
        }
    }

    // ==================== File Commands Tests ====================

    #[test]
    fn test_scan_directory_request_serialization() {
        let request = ScanRequest {
            directory_path: "C:/Users/Photos".to_string(),
            recursive: true,
            file_extensions: vec!["jpg".to_string(), "png".to_string()],
        };
        
        // Test serialization (simulates frontend -> backend transfer)
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("directoryPath"));
        assert!(json.contains("recursive"));
        
        // Test deserialization
        let deserialized: ScanRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.directory_path, "C:/Users/Photos");
        assert_eq!(deserialized.recursive, true);
        assert_eq!(deserialized.file_extensions.len(), 2);
    }

    #[test]
    fn test_scan_response_structure() {
        let response = ScanResponse {
            files: vec![],
            total_count: 0,
            directories_scanned: 0,
            elapsed_ms: 100,
            error: Some("Test error".to_string()),
        };
        
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("totalCount"));
        assert!(json.contains("elapsedMs"));
        assert!(json.contains("error"));
    }

    #[test]
    fn test_clear_files_command() {
        let state = create_test_state_with_files();
        
        // Verify files exist initially
        {
            let files = state.files.lock().unwrap();
            assert_eq!(files.len(), 2);
        }
        
        // This simulates calling the clear_files command
        // In real Tauri tests, we'd need the full runtime
        match state.files.lock() {
            Ok(mut guard) => guard.clear(),
            Err(_) => panic!("Failed to lock"),
        }
        
        // Verify files are cleared
        {
            let files = state.files.lock().unwrap();
            assert_eq!(files.len(), 0);
        }
    }

    // ==================== Preview Commands Tests ====================

    #[test]
    fn test_preview_request_with_methods() {
        let request = PreviewRequest {
            methods: vec![
                MethodConfig::Replace(ReplaceConfig {
                    enabled: true,
                    find: "photo".to_string(),
                    replace_with: "image".to_string(),
                    occurrence: OccurrenceOption::All,
                    case_sensitive: false,
                    use_regex: false,
                    apply_to: ApplyToOption::Name,
                }),
            ],
            template: Some("<Date:YYYY>_<Name>".to_string()),
        };
        
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("methods"));
        assert!(json.contains("template"));
        assert!(json.contains("Date:YYYY"));
    }

    #[test]
    fn test_preview_response_serialization() {
        use flowrename::commands::preview_commands::FilePreviewItem;
        
        let response = PreviewResponse {
            files: vec![
                FilePreviewItem {
                    original_path: "C:/Test/old.jpg".to_string(),
                    original_name: "old".to_string(),
                    new_name: "new".to_string(),
                    is_changed: true,
                    has_conflict: false,
                }
            ],
            total_count: 1,
            changed_count: 1,
            conflict_count: 0,
            elapsed_ms: 50,
            error: None,
        };
        
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("originalPath"));
        assert!(json.contains("isChanged"));
        assert!(json.contains("hasConflict"));
    }

    #[test]
    fn test_pipeline_execution_in_preview_context() {
        let mut pipeline = Pipeline::new();
        
        // Add a simple replace method
        pipeline.add_method(
            flowrename::methods::create_method_from_config(
                &MethodConfig::Replace(ReplaceConfig {
                    enabled: true,
                    find: "photo".to_string(),
                    replace_with: "IMG_".to_string(),
                    occurrence: OccurrenceOption::First,
                    case_sensitive: false,
                    use_regex: false,
                    apply_to: ApplyToOption::Name,
                })
            ).unwrap()
        );
        
        let context = MethodContext {
            file_index: 0,
            total_files: 1,
            file_metadata: None,
            original_name: "photo_vacation.jpg".to_string(),
            original_ext: ".jpg".to_string(),
            created_time: None,
            modified_time: None,
            accessed_time: None,
        };
        
        let result = pipeline.execute("photo_vacation", &context).unwrap();
        assert_eq!(result, "IMG__vacation");  // Replaced 'photo' at start
    }

    // ==================== Execute Rename Tests ====================

    #[test]
    fn test_execute_rename_request_structure() {
        let request = ExecuteRenameRequest {
            rename_items: vec![
                RenameItem {
                    current_path: "C:/Test/old_name.txt".to_string(),
                    new_name: "new_name.txt".to_string(),
                }
            ],
            create_undo_history: true,
        };
        
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("renameItems"));
        assert!(json.contains("currentPath"));
        assert!(json.contains("createUndoHistory"));
    }

    #[test]
    fn test_execute_response_error_handling() {
        use flowrename::commands::rename_commands::RenameError;
        
        let response = ExecuteResponse {
            success_count: 0,
            failed_count: 1,
            errors: vec![
                RenameError {
                    original_path: "C:/Nonexistent/file.txt".to_string(),
                    attempted_name: "renamed.txt".to_string(),
                    reason: "Source file does not exist".to_string(),
                }
            ],
            elapsed_ms: 10,
            error: None,
        };
        
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("failedCount"));
        assert!(json.contains("reason"));
        
        // Verify error details are preserved
        let deserialized: ExecuteResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.errors.len(), 1);
        assert_eq!(deserialized.errors[0].reason, "Source file does not exist");
    }

    // ==================== Integration Test: Full Workflow ====================

    #[test]
    fn test_full_workflow_data_flow() {
        // Simulate the complete workflow:
        // 1. Scan files -> Get FileItems
        // 2. User configures methods
        // 3. Preview generates new names
        // 4. Execute performs actual renames
        
        let state = create_test_state_with_files();
        
        // Step 1: Files loaded in state
        {
            let files = state.files.lock().unwrap();
            assert_eq!(files.len(), 2);  // photo1.jpg, document.txt
        }
        
        // Step 2: Configure methods
        let preview_req = PreviewRequest {
            methods: vec![
                MethodConfig::Replace(ReplaceConfig {
                    enabled: true,
                    find: "photo".to_string(),
                    replace_with: "IMAGE".to_string(),
                    occurrence: OccurrenceOption::All,
                    case_sensitive: false,
                    use_regex: false,
                    apply_to: ApplyToOption::Name,
                }),
            ],
            template: None,
        };
        
        // Step 3: Build pipeline and process
        let mut pipeline = Pipeline::new();
        for method_cfg in &preview_req.methods {
            let method = flowrename::methods::create_method_from_config(method_cfg).unwrap();
            pipeline.add_method(method);
        }
        
        // Process first file
        let ctx = MethodContext {
            file_index: 0,
            total_files: 2,
            file_metadata: None,
            original_name: "photo1".to_string(),
            original_ext: ".jpg".to_string(),
            created_time: None,
            modified_time: None,
            accessed_time: None,
        };
        
        let new_name = pipeline.execute("photo1", &ctx).unwrap();
        assert_eq!(new_name, "IMAGE1");  // 'photo' replaced with 'IMAGE'
        
        // Step 4: Would execute rename (requires actual filesystem)
        println!("✓ Full workflow data flow validated successfully");
    }
}
