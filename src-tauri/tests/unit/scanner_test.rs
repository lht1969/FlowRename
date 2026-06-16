#[cfg(test)]
mod tests {
    use std::fs;
    use tempfile::TempDir;
    
    use flowrename::file_manager::FileManager;

    #[test]
    fn test_scan_single_file() {
        // Create a temporary directory with a test file
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "content").unwrap();
        
        // Load the single file
        let files = FileManager::load_files(vec![file_path], false, None);
        
        // Verify success and correct count
        assert!(files.is_ok());
        let items = files.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].original_name, "test");
        assert_eq!(items[0].original_ext, ".txt");
    }

    #[test]
    fn test_scan_folder_recursive() {
        // Create nested directory structure
        let dir = TempDir::new().unwrap();
        fs::create_dir_all(dir.path().join("sub")).unwrap();
        fs::write(dir.path().join("a.txt"), "").unwrap();
        fs::write(dir.path().join("sub/b.txt"), "").unwrap();
        fs::create_dir_all(dir.path().join("sub/deep")).unwrap();
        fs::write(dir.path().join("sub/deep/c.txt"), "").unwrap();
        
        // Load directory recursively
        let files = FileManager::load_files(vec![dir.path().to_path_buf()], true, None);
        
        // Should find all 3 files
        assert!(files.is_ok());
        let items = files.unwrap();
        assert_eq!(items.len(), 3); // a.txt, sub/b.txt, sub/deep/c.txt
    }

    #[test]
    fn test_scan_folder_non_recursive() {
        let dir = TempDir::new().unwrap();
        fs::create_dir_all(dir.path().join("sub")).unwrap();
        fs::write(dir.path().join("a.jpg"), "").unwrap();
        fs::write(dir.path().join("sub/b.jpg"), "").unwrap();
        
        // Load without recursion
        let files = FileManager::load_files(vec![dir.path().to_path_buf()], false, None);
        
        // Should only find top-level file
        assert!(files.is_ok());
        let items = files.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].original_name, "a");
    }

    #[test]
    fn test_filter_by_extension() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("photo.jpg"), "").unwrap();
        fs::write(dir.path().join("doc.txt"), "").unwrap();
        fs::write(dir.path().join("image.png"), "").unwrap();
        
        // Filter for image files only
        let filter = vec!["jpg".to_string(), "png".to_string()];
        let files = FileManager::load_files(vec![dir.path().to_path_buf()], false, Some(filter));
        
        // Should only include jpg and png files
        let items = files.unwrap();
        assert_eq!(items.len(), 2);
        let names: Vec<&str> = items.iter().map(|f| f.original_name.as_str()).collect();
        assert!(names.contains(&"photo"));
        assert!(names.contains(&"image"));
    }

    #[test]
    fn test_file_size_and_timestamps() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("data.bin");
        let content = b"This is some test data";
        fs::write(&file_path, content).unwrap();
        
        let files = FileManager::load_files(vec![file_path], false, None);
        let items = files.unwrap();
        
        let item = &items[0];
        assert_eq!(item.file_size, content.len() as u64);
        assert!(item.created_time <= chrono::Utc::now());
        assert!(item.modified_time <= chrono::Utc::now());
    }
}
