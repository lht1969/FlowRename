#[cfg(test)]
mod tests {
    use flowrename::models::{FileItem, FileStatus, ImageMetadata, AudioMetadata, ApplyToOption};

    #[test]
    fn test_file_item_creation() {
        let item = FileItem::new(
            std::path::PathBuf::from("test/photo.jpg"),
            "photo".to_string(),
            ".jpg".to_string(),
        );
        
        assert_eq!(item.original_name, "photo");
        assert_eq!(item.original_ext, ".jpg");
        assert_eq!(item.status, FileStatus::Pending);
    }

    #[test]
    fn test_full_original_name() {
        let item = FileItem::new(
            std::path::PathBuf::from("test/document.pdf"),
            "document".to_string(),
            ".pdf".to_string(),
        );
        
        assert_eq!(item.full_original_name(), "document.pdf");
    }

    #[test]
    fn test_file_status_variants() {
        // Test all status variants exist and can be created
        let _pending = FileStatus::Pending;
        let _success = FileStatus::Success;
        let failed = FileStatus::Failed("Test error".to_string());
        
        match failed {
            FileStatus::Failed(msg) => assert_eq!(msg, "Test error"),
            _ => panic!("Expected Failed variant")
        }
    }

    #[test]
    fn test_image_metadata_default() {
        let meta: ImageMetadata = Default::default();
        assert!(meta.width.is_none());
        assert!(meta.height.is_none());
        assert!(meta.make.is_none());
    }

    #[test]
    fn test_image_metadata_with_values() {
        let meta = ImageMetadata {
            width: Some(1920),
            height: Some(1080),
            make: Some("Canon".to_string()),
            ..Default::default()
        };
        
        assert_eq!(meta.width.unwrap(), 1920);
        assert_eq!(meta.height.unwrap(), 1080);
        assert_eq!(meta.make.unwrap(), "Canon");
    }

    #[test]
    fn test_audio_metadata() {
        let audio = AudioMetadata {
            title: Some("Song Title".to_string()),
            artist: Some("Artist Name".to_string()),
            album: Some("Album Name".to_string()),
            year: Some(2024),
            track_number: Some(1),
            genre: Some("Pop".to_string()),
            disc_number: None,
            total_discs: None,
            duration: Some(180),
            bitrate: Some(320),
            recording_date: None,
            legacy_date: None,
        };
        
        assert_eq!(audio.title.unwrap(), "Song Title");
        assert_eq!(audio.artist.unwrap(), "Artist Name");
        assert_eq!(audio.year.unwrap(), 2024);
        assert_eq!(audio.duration.unwrap(), 180);
    }

    #[test]
    fn test_apply_to_option_defaults_to_both() {
        let default: ApplyToOption = Default::default();
        matches!(default, ApplyToOption::Both);
    }
}
