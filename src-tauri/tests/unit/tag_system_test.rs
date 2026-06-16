#[cfg(test)]
mod tests {
    use flowrename::tag_system::{TagParser, TagEvaluator};
    use flowrename::method_engine::MethodContext;
    use chrono::Utc;

    fn create_context() -> MethodContext {
        MethodContext {
            file_index: 2,
            total_files: 5,
            file_metadata: None,
            original_name: "vacation_photo".to_string(),
            original_ext: ".jpg".to_string(),
            created_time: None,
            modified_time: None,
            accessed_time: None,
        }
    }

    // ==================== Tag Parser Tests ====================

    #[test]
    fn test_parse_simple_tag() {
        let parser = TagParser::new();
        let template = "<Name>";
        let tokens = parser.parse(template);
        
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].tag_name, "Name");
        assert_eq!(tokens[0].modifiers.is_empty(), true);
    }

    #[test]
    fn test_parse_tag_with_modifier() {
        let parser = TagParser::new();
        let template = "<Date:YYYYMMDD>";
        let tokens = parser.parse(template);
        
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].tag_name, "Date");
        assert_eq!(tokens[0].modifiers, vec!["YYYYMMDD".to_string()]);
    }

    #[test]
    fn test_parse_multiple_tags() {
        let parser = TagParser::new();
        let template = "<Date:YYYY>_<Inc:3>_photo";
        let tokens = parser.parse(template);
        
        assert_eq!(tokens.len(), 2);  // Two tags (text between is not a tag)
        assert_eq!(tokens[0].tag_name, "Date");
        assert_eq!(tokens[1].tag_name, "Inc");
    }

    #[test]
    fn test_parse_empty_template() {
        let parser = TagParser::new();
        let tokens = parser.parse("simple text without tags");
        
        assert_eq!(tokens.len(), 0);  // No tags found
    }

    #[test]
    fn test_parse_malformed_tag_returns_empty() {
        let parser = TagParser::new();
        let template = "<unclosed tag";
        let tokens = parser.parse(template);
        
        // Malformed tags should be ignored or handled gracefully
        assert!(tokens.is_empty());
    }

    // ==================== Basic Tags Evaluation Tests ====================

    #[test]
    fn test_evaluate_name_tag() {
        let evaluator = TagEvaluator::new();
        let ctx = create_context();
        
        let result = evaluator.evaluate_tag("Name", &[], &ctx).unwrap();
        assert_eq!(result, "vacation_photo");
    }

    #[test]
    fn test_evaluate_ext_tag() {
        let evaluator = TagEvaluator::new();
        let ctx = create_context();
        
        let result = evaluator.evaluate_tag("Ext", &[], &ctx).unwrap();
        assert_eq!(result, ".jpg");
    }

    #[test]
    fn test_evaluate_index_tag() {
        let evaluator = TagEvaluator::new();
        let ctx = create_context();  // file_index = 2
        
        let result = evaluator.evaluate_tag("Index", &[], &ctx).unwrap();
        assert_eq!(result, "3");  // 1-based index
    }

    // ==================== Timestamp Tags Tests ====================

    #[test]
    fn test_evaluate_date_tag_with_format() {
        let evaluator = TagEvaluator::new();
        let ctx = create_context();
        
        let result = evaluator.evaluate_tag("Date", &[String::from("YYYY")], &ctx).unwrap();
        // Should return current year in YYYY format
        let year = Utc::now().format("%Y").to_string();
        assert_eq!(result, year);
    }

    #[test]
    fn test_evaluate_time_tag() {
        let evaluator = TagEvaluator::new();
        let ctx = create_context();
        
        let result = evaluator.evaluate_tag("Time", &[String::from("HHmmss")], &ctx).unwrap();
        // Should be 6 digits (HHmmss format)
        assert_eq!(result.len(), 6);
        assert!(result.chars().all(|c: char| c.is_ascii_digit()));
    }

    // ==================== Sequence Tags Tests ====================

    #[test]
    fn test_evaluate_inc_tag_basic() {
        let evaluator = TagEvaluator::new();
        let ctx = create_context();  // file_index = 2
        
        let result = evaluator.evaluate_tag("Inc", &[], &ctx).unwrap();
        assert_eq!(result, "3");  // Default: 1-based, no padding
    }

    #[test]
    fn test_evaluate_inc_tag_with_padding() {
        let evaluator = TagEvaluator::new();
        let ctx = create_context();  // file_index = 2
        
        let result = evaluator.evaluate_tag("Inc", &[String::from("4")], &ctx).unwrap();
        assert_eq!(result, "0003");  // Padded to 4 digits
    }

    #[test]
    fn test_evaluate_inc_tag_start_value() {
        let evaluator = TagEvaluator::new();
        let mut ctx = create_context();
        ctx.file_index = 0;  // First file
        
        let result = evaluator.evaluate_tag("Inc", &[String::from("3"), String::from("100")], &ctx).unwrap();
        assert_eq!(result, "100");  // Start from 100, padded to 3 digits
    }

    // ==================== Full Template Evaluation Tests ====================

    #[test]
    fn test_evaluate_full_template() {
        let evaluator = TagEvaluator::new();
        let ctx = create_context();
        
        let template = "<Date:YYYY>-<Inc:3>_<Name><Ext>";
        let result = evaluator.evaluate_template(&template, &ctx).unwrap();
        
        // Should contain date, sequence number, original name and extension
        assert!(result.starts_with(|c: char| c.is_ascii_digit()));  // Starts with year
        assert!(result.contains("-"));  // Separator present
        assert!(result.contains("_"));  // Separator present
        assert!(result.ends_with(".jpg"));  // Original extension preserved
    }

    #[test]
    fn test_evaluate_template_without_tags() {
        let evaluator = TagEvaluator::new();
        let ctx = create_context();
        
        let static_text = "static_filename";
        let result = evaluator.evaluate_template(static_text, &ctx).unwrap();
        
        assert_eq!(result, static_text);  // Should return unchanged
    }

    #[test]
    fn test_unknown_tag_returns_error() {
        let evaluator = TagEvaluator::new();
        let ctx = create_context();
        
        let result = evaluator.evaluate_tag("UnknownTag", &[], &ctx);
        assert!(result.is_err());  // Should fail for unknown tags
    }

    // ==================== Video Tags Tests ====================

    fn create_video_context() -> MethodContext {
        use flowrename::models::{FileMetadata, VideoMetadata};
        let mut metadata = FileMetadata::default();
        metadata.video = Some(VideoMetadata {
            width: Some(1920),
            height: Some(1080),
            frame_rate: Some(29.97),
            duration_secs: Some(3661.5),
            title: Some("Test Video".to_string()),
            genre: Some("Documentary".to_string()),
            creation_date: None,
            codec: Some("H.264".to_string()),
            bit_rate: Some(5000),
        });
        MethodContext {
            file_index: 0,
            total_files: 1,
            file_metadata: Some(metadata),
            original_name: "test_video".to_string(),
            original_ext: ".mp4".to_string(),
            created_time: None,
            modified_time: None,
            accessed_time: None,
        }
    }

    #[test]
    fn test_evaluate_vid_width_tag() {
        let evaluator = TagEvaluator::new();
        let ctx = create_video_context();
        let result = evaluator.evaluate_tag("VidWidth", &[], &ctx).unwrap();
        assert_eq!(result, "1920");
    }

    #[test]
    fn test_evaluate_vid_height_tag() {
        let evaluator = TagEvaluator::new();
        let ctx = create_video_context();
        let result = evaluator.evaluate_tag("VidHeight", &[], &ctx).unwrap();
        assert_eq!(result, "1080");
    }

    #[test]
    fn test_evaluate_vid_frame_rate_tag() {
        let evaluator = TagEvaluator::new();
        let ctx = create_video_context();
        let result = evaluator.evaluate_tag("VidFrameRate", &[], &ctx).unwrap();
        assert_eq!(result, "29.97");
    }

    #[test]
    fn test_evaluate_vid_duration_tag() {
        let evaluator = TagEvaluator::new();
        let ctx = create_video_context();
        let result = evaluator.evaluate_tag("VidDuration", &[], &ctx).unwrap();
        assert_eq!(result, "01-01-01");
    }

    #[test]
    fn test_evaluate_vid_duration_hhmmss_tag() {
        let evaluator = TagEvaluator::new();
        let ctx = create_video_context();
        let result = evaluator.evaluate_tag("VidDuration", &[String::from("HH:MM:SS")], &ctx).unwrap();
        assert_eq!(result, "01:01:01");
    }

    #[test]
    fn test_evaluate_vid_duration_secs_tag() {
        let evaluator = TagEvaluator::new();
        let ctx = create_video_context();
        let result = evaluator.evaluate_tag("VidDurationSec", &[], &ctx).unwrap();
        assert_eq!(result, "3662");
    }

    #[test]
    fn test_evaluate_vid_title_tag() {
        let evaluator = TagEvaluator::new();
        let ctx = create_video_context();
        let result = evaluator.evaluate_tag("VidTitle", &[], &ctx).unwrap();
        assert_eq!(result, "Test Video");
    }

    #[test]
    fn test_evaluate_vid_codec_tag() {
        let evaluator = TagEvaluator::new();
        let ctx = create_video_context();
        let result = evaluator.evaluate_tag("VidCodec", &[], &ctx).unwrap();
        assert_eq!(result, "H.264");
    }

    #[test]
    fn test_evaluate_vid_bitrate_tag() {
        let evaluator = TagEvaluator::new();
        let ctx = create_video_context();
        let result = evaluator.evaluate_tag("VidBitRate", &[], &ctx).unwrap();
        assert_eq!(result, "5000");
    }

    // ==================== Alias Mapping Tests ====================

    #[test]
    fn test_exif_alias_maps_to_img() {
        let evaluator = TagEvaluator::new();
        let ctx = create_context();
        // ExifWidth 是 ImgWidth 的别名，两者应产生相同结果
        let result_img = evaluator.evaluate_tag("ImgWidth", &[], &ctx);
        let result_exif = evaluator.evaluate_tag("ExifWidth", &[], &ctx);
        assert_eq!(result_img.is_err(), result_exif.is_err());
    }

    #[test]
    fn test_id3_alias_maps_to_aud() {
        let evaluator = TagEvaluator::new();
        let ctx = create_context();
        // Id3Title 是 AudTitle 的别名
        let result_aud = evaluator.evaluate_tag("AudTitle", &[], &ctx);
        let result_id3 = evaluator.evaluate_tag("Id3Title", &[], &ctx);
        assert_eq!(result_aud.is_err(), result_id3.is_err());
    }

    #[test]
    fn test_vid_fps_alias() {
        let evaluator = TagEvaluator::new();
        let ctx = create_video_context();
        let result1 = evaluator.evaluate_tag("VidFrameRate", &[], &ctx).unwrap();
        let result2 = evaluator.evaluate_tag("VidFps", &[], &ctx).unwrap();
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_vid_bitrate_alias() {
        let evaluator = TagEvaluator::new();
        let ctx = create_video_context();
        let result1 = evaluator.evaluate_tag("VidBitRate", &[], &ctx).unwrap();
        let result2 = evaluator.evaluate_tag("VidBitrate", &[], &ctx).unwrap();
        assert_eq!(result1, result2);
    }

    // ==================== Audio Extended Tags Tests ====================

    fn create_audio_context() -> MethodContext {
        use flowrename::models::{FileMetadata, AudioMetadata};
        let mut metadata = FileMetadata::default();
        metadata.audio = Some(AudioMetadata {
            title: Some("Test Song".to_string()),
            artist: Some("Test Artist".to_string()),
            album: Some("Test Album".to_string()),
            year: Some(2024),
            track_number: Some(5),
            genre: Some("Pop".to_string()),
            disc_number: Some(2),
            total_discs: Some(3),
            duration: Some(245),
            bitrate: Some(320),
            recording_date: None,
            legacy_date: None,
        });
        MethodContext {
            file_index: 0,
            total_files: 1,
            file_metadata: Some(metadata),
            original_name: "test_song".to_string(),
            original_ext: ".mp3".to_string(),
            created_time: None,
            modified_time: None,
            accessed_time: None,
        }
    }

    #[test]
    fn test_evaluate_aud_duration_tag() {
        let evaluator = TagEvaluator::new();
        let ctx = create_audio_context();
        let result = evaluator.evaluate_tag("AudDuration", &[], &ctx).unwrap();
        assert_eq!(result, "04-05");
    }

    #[test]
    fn test_evaluate_aud_duration_secs_tag() {
        let evaluator = TagEvaluator::new();
        let ctx = create_audio_context();
        let result = evaluator.evaluate_tag("AudDurationSec", &[], &ctx).unwrap();
        assert_eq!(result, "245");
    }

    #[test]
    fn test_evaluate_aud_bitrate_tag() {
        let evaluator = TagEvaluator::new();
        let ctx = create_audio_context();
        let result = evaluator.evaluate_tag("AudBitRate", &[], &ctx).unwrap();
        assert_eq!(result, "320");
    }

    #[test]
    fn test_evaluate_aud_disc_tag() {
        let evaluator = TagEvaluator::new();
        let ctx = create_audio_context();
        let result = evaluator.evaluate_tag("AudDisc", &[], &ctx).unwrap();
        assert_eq!(result, "2");
    }

    #[test]
    fn test_evaluate_aud_disc_with_padding() {
        let evaluator = TagEvaluator::new();
        let ctx = create_audio_context();
        let result = evaluator.evaluate_tag("AudDisc", &[String::from("2")], &ctx).unwrap();
        assert_eq!(result, "02");
    }
}
