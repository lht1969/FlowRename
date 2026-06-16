#[cfg(test)]
mod tests {
    use flowrename::method_engine::{Method, MethodContext, Pipeline, sanitize_filename};
    use flowrename::models::{
        ApplyToOption, CaseLocation, CaseType, MethodType, OccurrenceOption,
        AddPosition, ReplaceConfig, AddConfig, RemoveConfig, RemovePosition, NewCaseConfig
    };
    use flowrename::methods::p0_methods::{ReplaceMethod, AddMethod, RemoveMethod, NewCaseMethod, NewNameMethodAdapter};
    use flowrename::models::method_config::NewNameConfig;
    use flowrename::models::{FileMetadata, VideoMetadata};

    fn default_context() -> MethodContext {
        MethodContext::default()
    }

    fn jpg_context() -> MethodContext {
        MethodContext {
            original_ext: ".jpg".to_string(),
            ..Default::default()
        }
    }

    fn new_case_config(new_case: CaseType, location: CaseLocation, apply_to: ApplyToOption) -> NewCaseConfig {
        NewCaseConfig {
            enabled: true,
            new_case,
            location,
            apply_to,
        }
    }

    // ==================== Replace Method Tests ====================

    #[test]
    fn test_replace_simple_text() {
        let method = ReplaceMethod::new(ReplaceConfig {
            enabled: true,
            find: "old".to_string(),
            replace_with: "new".to_string(),
            occurrence: OccurrenceOption::All,
            case_sensitive: false,
            use_regex: false,
            apply_to: ApplyToOption::Name,
        });

        assert!(method.validate().is_ok());
        let result = method.apply("old_file_old.txt", &default_context()).unwrap();
        assert_eq!(result, "new_file_new.txt");
    }

    #[test]
    fn test_replace_first_occurrence_only() {
        let method = ReplaceMethod::new(ReplaceConfig {
            enabled: true,
            find: "x".to_string(),
            replace_with: "y".to_string(),
            occurrence: OccurrenceOption::First,
            case_sensitive: false,
            use_regex: false,
            apply_to: ApplyToOption::Name,
        });

        let result = method.apply("xxx_file", &default_context()).unwrap();
        assert_eq!(result, "yxx_file");
    }

    #[test]
    fn test_replace_case_sensitive() {
        let method = ReplaceMethod::new(ReplaceConfig {
            enabled: true,
            find: "Hello".to_string(),
            replace_with: "Hi".to_string(),
            occurrence: OccurrenceOption::All,
            case_sensitive: true,
            use_regex: false,
            apply_to: ApplyToOption::Name,
        });

        let result = method.apply("Hello hello HELLO", &default_context()).unwrap();
        assert_eq!(result, "Hi hello HELLO");
    }

    #[test]
    fn test_replace_with_regex() {
        let method = ReplaceMethod::new(ReplaceConfig {
            enabled: true,
            find: r"\d{3}".to_string(),  // Match exactly 3 digits
            replace_with: "XXX".to_string(),
            occurrence: OccurrenceOption::All,
            case_sensitive: false,
            use_regex: true,
            apply_to: ApplyToOption::Name,
        });

        let result = method.apply("file_123_test_456", &default_context()).unwrap();
        assert_eq!(result, "file_XXX_test_XXX");
    }

    #[test]
    fn test_replace_empty_find_should_fail() {
        let method = ReplaceMethod::new(ReplaceConfig {
            enabled: true,
            find: "".to_string(),
            replace_with: "something".to_string(),
            occurrence: OccurrenceOption::All,
            case_sensitive: false,
            use_regex: false,
            apply_to: ApplyToOption::Name,
        });

        assert!(method.validate().is_err());  // Empty search pattern should fail validation
    }

    // ==================== Add Method Tests ====================

    #[test]
    fn test_add_at_start() {
        let method = AddMethod::new(AddConfig {
            enabled: true,
            text: "IMG_".to_string(),
            position: AddPosition::Start,
            custom_index: None,
            backwards: false,
            apply_to: ApplyToOption::Name,
        });

        let result = method.apply("photo.jpg", &jpg_context()).unwrap();
        assert_eq!(result, "IMG_photo.jpg");
    }

    #[test]
    fn test_add_at_end() {
        let method = AddMethod::new(AddConfig {
            enabled: true,
            text: "_v2".to_string(),
            position: AddPosition::End,
            custom_index: None,
            backwards: false,
            apply_to: ApplyToOption::Name,
        });

        let result = method.apply("photo.jpg", &jpg_context()).unwrap();
        assert_eq!(result, "photo_v2.jpg");
    }

    #[test]
    fn test_add_before_extension() {
        let method = AddMethod::new(AddConfig {
            enabled: true,
            text: "_edit".to_string(),
            position: AddPosition::BeforeExt,
            custom_index: None,
            backwards: false,
            apply_to: ApplyToOption::Name,
        });

        let result = method.apply("photo.jpg", &jpg_context()).unwrap();
        assert_eq!(result, "photo_edit.jpg");
    }

    #[test]
    fn test_add_after_extension() {
        let method = AddMethod::new(AddConfig {
            enabled: true,
            text: ".backup".to_string(),
            position: AddPosition::AfterExt,
            custom_index: None,
            backwards: false,
            apply_to: ApplyToOption::Name,
        });

        let result = method.apply("photo.jpg", &jpg_context()).unwrap();
        assert_eq!(result, "photo.backup");
    }

    #[test]
    fn test_add_empty_text_should_fail() {
        let method = AddMethod::new(AddConfig {
            enabled: true,
            text: "".to_string(),
            position: AddPosition::Start,
            custom_index: None,
            backwards: false,
            apply_to: ApplyToOption::Name,
        });

        assert!(method.validate().is_err());
    }

    // ==================== Remove Method Tests ====================

    #[test]
    fn test_remove_by_position() {
        let method = RemoveMethod::new(RemoveConfig {
            enabled: true,
            count: 3,
            position: RemovePosition::Start,
            apply_to: ApplyToOption::Name,
        });

        let result = method.apply("IMG_photo.jpg", &default_context()).unwrap();
        // Position removal removes first 3 characters by default (basic implementation)
        assert!(!result.starts_with("IMG"));
    }

    #[test]
    fn test_remove_method_name_and_type() {
        let method = RemoveMethod::new(RemoveConfig {
            enabled: true,
            count: 3,
            position: RemovePosition::Start,
            apply_to: ApplyToOption::Name,
        });

        assert_eq!(method.name(), "Remove");
        assert_eq!(method.method_type(), MethodType::Remove);
    }

    // ==================== NewCase Method Tests ====================

    #[test]
    fn test_newcase_uppercase_all() {
        let method = NewCaseMethod::new(new_case_config(CaseType::Upper, CaseLocation::All, ApplyToOption::Name));

        let result = method.apply("hello world.jpg", &jpg_context()).unwrap();
        assert_eq!(result, "HELLO WORLD.jpg");
    }

    #[test]
    fn test_newcase_lowercase_all() {
        let method = NewCaseMethod::new(new_case_config(CaseType::Lower, CaseLocation::All, ApplyToOption::Name));

        let result = method.apply("HELLO WORLD.jpg", &jpg_context()).unwrap();
        assert_eq!(result, "hello world.jpg");
    }

    #[test]
    fn test_newcase_title_case() {
        let method = NewCaseMethod::new(new_case_config(CaseType::Title, CaseLocation::All, ApplyToOption::Name));

        let result = method.apply("hello world from rust.jpg", &jpg_context()).unwrap();
        assert_eq!(result, "Hello World From Rust.jpg");
    }

    #[test]
    fn test_newcase_title_case_with_hyphen_separator() {
        let method = NewCaseMethod::new(new_case_config(CaseType::Title, CaseLocation::All, ApplyToOption::Name));

        let result = method.apply("hello-world-from-rust.jpg", &jpg_context()).unwrap();
        assert_eq!(result, "Hello-World-From-Rust.jpg");
    }

    #[test]
    fn test_newcase_title_case_with_underscore_separator() {
        let method = NewCaseMethod::new(new_case_config(CaseType::Title, CaseLocation::All, ApplyToOption::Name));

        let result = method.apply("hello_world_from_rust.jpg", &jpg_context()).unwrap();
        assert_eq!(result, "Hello_World_From_Rust.jpg");
    }

    #[test]
    fn test_newcase_title_case_with_mixed_separators() {
        let method = NewCaseMethod::new(new_case_config(CaseType::Title, CaseLocation::All, ApplyToOption::Name));

        let result = method.apply("hello-world_from rust.jpg", &jpg_context()).unwrap();
        assert_eq!(result, "Hello-World_From Rust.jpg");
    }

    #[test]
    fn test_newcase_sentence_case() {
        let method = NewCaseMethod::new(new_case_config(CaseType::Sentence, CaseLocation::All, ApplyToOption::Name));

        let result = method.apply("hello world. this is a test.jpg", &jpg_context()).unwrap();
        assert_eq!(result, "Hello world. This is a test.jpg");
    }

    #[test]
    fn test_newcase_inverted_case() {
        let method = NewCaseMethod::new(new_case_config(CaseType::Inverted, CaseLocation::All, ApplyToOption::Name));

        let result = method.apply("HeLLo WoRLd.jpg", &jpg_context()).unwrap();
        assert_eq!(result, "hEllO wOrlD.jpg");
    }

    // ==================== NewCase FirstLetter Location Tests ====================

    #[test]
    fn test_newcase_upper_first_letter() {
        let method = NewCaseMethod::new(new_case_config(CaseType::Upper, CaseLocation::FirstLetter, ApplyToOption::Name));

        let result = method.apply("hello world.jpg", &jpg_context()).unwrap();
        assert_eq!(result, "Hello world.jpg");
    }

    #[test]
    fn test_newcase_lower_first_letter() {
        let method = NewCaseMethod::new(new_case_config(CaseType::Lower, CaseLocation::FirstLetter, ApplyToOption::Name));

        let result = method.apply("HELLO WORLD.jpg", &jpg_context()).unwrap();
        assert_eq!(result, "hELLO WORLD.jpg");
    }

    #[test]
    fn test_newcase_title_first_letter() {
        let method = NewCaseMethod::new(new_case_config(CaseType::Title, CaseLocation::FirstLetter, ApplyToOption::Name));

        let result = method.apply("hello world.jpg", &jpg_context()).unwrap();
        assert_eq!(result, "Hello world.jpg");
    }

    #[test]
    fn test_newcase_sentence_first_letter() {
        let method = NewCaseMethod::new(new_case_config(CaseType::Sentence, CaseLocation::FirstLetter, ApplyToOption::Name));

        let result = method.apply("hello. world. test.jpg", &jpg_context()).unwrap();
        assert_eq!(result, "Hello. world. test.jpg");
    }

    #[test]
    fn test_newcase_inverted_first_letter() {
        let method = NewCaseMethod::new(new_case_config(CaseType::Inverted, CaseLocation::FirstLetter, ApplyToOption::Name));

        let result = method.apply("HeLLo WoRLd.jpg", &jpg_context()).unwrap();
        assert_eq!(result, "heLLo WoRLd.jpg");
    }

    #[test]
    fn test_newcase_first_letter_empty_string() {
        let method = NewCaseMethod::new(new_case_config(CaseType::Upper, CaseLocation::FirstLetter, ApplyToOption::Name));

        let result = method.apply(".jpg", &jpg_context()).unwrap();
        assert_eq!(result, ".jpg");
    }

    // ==================== NewCase FirstLetter vs All 差异性测试 ====================

    #[test]
    fn test_newcase_upper_first_letter_differs_from_all() {
        let method_all = NewCaseMethod::new(new_case_config(CaseType::Upper, CaseLocation::All, ApplyToOption::Name));
        let method_first = NewCaseMethod::new(new_case_config(CaseType::Upper, CaseLocation::FirstLetter, ApplyToOption::Name));

        let ctx = jpg_context();
        let result_all = method_all.apply("hello world.jpg", &ctx).unwrap();
        let result_first = method_first.apply("hello world.jpg", &ctx).unwrap();

        assert_eq!(result_all, "HELLO WORLD.jpg");
        assert_eq!(result_first, "Hello world.jpg");
        assert_ne!(result_all, result_first, "All 和 FirstLetter 结果必须不同");
    }

    #[test]
    fn test_newcase_lower_first_letter_differs_from_all() {
        let method_all = NewCaseMethod::new(new_case_config(CaseType::Lower, CaseLocation::All, ApplyToOption::Name));
        let method_first = NewCaseMethod::new(new_case_config(CaseType::Lower, CaseLocation::FirstLetter, ApplyToOption::Name));

        let ctx = jpg_context();
        let result_all = method_all.apply("HELLO WORLD.jpg", &ctx).unwrap();
        let result_first = method_first.apply("HELLO WORLD.jpg", &ctx).unwrap();

        assert_eq!(result_all, "hello world.jpg");
        assert_eq!(result_first, "hELLO WORLD.jpg");
        assert_ne!(result_all, result_first, "All 和 FirstLetter 结果必须不同");
    }

    // ==================== NewCase EveryWordFirstLetter Location Tests ====================

    #[test]
    fn test_newcase_upper_every_word_first_letter() {
        let method = NewCaseMethod::new(new_case_config(CaseType::Upper, CaseLocation::EveryWordFirstLetter, ApplyToOption::Name));

        let result = method.apply("hello world.jpg", &jpg_context()).unwrap();
        assert_eq!(result, "Hello World.jpg");
    }

    #[test]
    fn test_newcase_lower_every_word_first_letter() {
        let method = NewCaseMethod::new(new_case_config(CaseType::Lower, CaseLocation::EveryWordFirstLetter, ApplyToOption::Name));

        let result = method.apply("HELLO WORLD.jpg", &jpg_context()).unwrap();
        assert_eq!(result, "hELLO wORLD.jpg");
    }

    #[test]
    fn test_newcase_inverted_every_word_first_letter() {
        let method = NewCaseMethod::new(new_case_config(CaseType::Inverted, CaseLocation::EveryWordFirstLetter, ApplyToOption::Name));

        let result = method.apply("Hello World.jpg", &jpg_context()).unwrap();
        assert_eq!(result, "hello world.jpg");
    }

    // ==================== Pipeline Integration Tests ====================

    #[test]
    fn test_pipeline_with_multiple_p0_methods() {
        let mut pipeline = Pipeline::new();

        pipeline.add_method(Box::new(AddMethod::new(AddConfig {
            enabled: true,
            text: "IMG_".to_string(),
            position: AddPosition::Start,
            custom_index: None,
            backwards: false,
            apply_to: ApplyToOption::Name,
        })));

        pipeline.add_method(Box::new(ReplaceMethod::new(ReplaceConfig {
            enabled: true,
            find: " ".to_string(),
            replace_with: "_".to_string(),
            occurrence: OccurrenceOption::All,
            case_sensitive: false,
            use_regex: false,
            apply_to: ApplyToOption::Name,
        })));

        pipeline.add_method(Box::new(NewCaseMethod::new(new_case_config(CaseType::Upper, CaseLocation::All, ApplyToOption::Name))));

        let ctx = MethodContext {
            original_ext: ".jpg".to_string(),
            ..Default::default()
        };
        let result = pipeline.execute("my photo file.jpg", &ctx).unwrap();

        assert_eq!(result, "IMG_MY_PHOTO_FILE.jpg");
    }

    // ==================== NewName Method Integration Tests ====================

    fn create_video_context() -> MethodContext {
        let mut metadata = FileMetadata::default();
        metadata.video = Some(VideoMetadata {
            width: Some(1920),
            height: Some(1080),
            frame_rate: Some(29.97),
            duration_secs: Some(3661.5),
            title: Some("Sample Video".to_string()),
            genre: Some("Tutorial".to_string()),
            creation_date: None,
            codec: Some("H.264".to_string()),
            bit_rate: Some(5000),
        });

        MethodContext {
            file_index: 0,
            total_files: 5,
            file_metadata: Some(metadata),
            original_name: "vacation_2024".to_string(),
            original_ext: ".mp4".to_string(),
            ..Default::default()
        }
    }

    #[test]
    fn test_newname_with_video_tags() {
        let method = NewNameMethodAdapter::new(NewNameConfig {
            enabled: true,
            template: "<VidWidth>x<VidHeight>_<Name>".to_string(),
            apply_to: ApplyToOption::Name,
        });

        assert!(method.validate().is_ok());
        let ctx = create_video_context();
        let result = method.apply("vacation_2024.mp4", &ctx).unwrap();
        assert_eq!(result, "1920x1080_vacation_2024");
    }

    #[test]
    fn test_newname_with_multiple_video_tags() {
        let method = NewNameMethodAdapter::new(NewNameConfig {
            enabled: true,
            template: "<VidCodec>_<VidFps>_<VidBitrate>".to_string(),
            apply_to: ApplyToOption::Name,
        });

        let ctx = create_video_context();
        let result = method.apply("vacation_2024.mp4", &ctx).unwrap();
        assert_eq!(result, "H.264_29.97_5000");
    }

    #[test]
    fn test_newname_with_vid_duration_hhmmss() {
        let method = NewNameMethodAdapter::new(NewNameConfig {
            enabled: true,
            template: "<VidDuration>_<Name>".to_string(),
            apply_to: ApplyToOption::Name,
        });

        let ctx = create_video_context();
        let result = method.apply("vacation_2024.mp4", &ctx).unwrap();
        assert_eq!(result, "01-01-01_vacation_2024");
    }

    #[test]
    fn test_newname_video_tags_not_evaluated_as_literal_angle_brackets() {
        let method = NewNameMethodAdapter::new(NewNameConfig {
            enabled: true,
            template: "<VidWidth>x<VidHeight>".to_string(),
            apply_to: ApplyToOption::Name,
        });

        let ctx = create_video_context();
        let result = method.apply("test.mp4", &ctx).unwrap();
        assert!(!result.contains('<'), "标签不应残留尖括号: {result}");
        assert!(!result.contains('>'), "标签不应残留尖括号: {result}");
        assert_eq!(result, "1920x1080");
    }

    #[test]
    fn test_newname_basic_tags_still_work() {
        let method = NewNameMethodAdapter::new(NewNameConfig {
            enabled: true,
            template: "<Index>_<Name>_<Cnt:3>".to_string(),
            apply_to: ApplyToOption::Name,
        });

        let ctx = MethodContext {
            file_index: 4,
            total_files: 10,
            original_name: "photo".to_string(),
            original_ext: ".jpg".to_string(),
            ..Default::default()
        };
        let result = method.apply("photo.jpg", &ctx).unwrap();
        assert_eq!(result, "5_photo_010");
    }

    #[test]
    fn test_newname_with_date_tag() {
        let method = NewNameMethodAdapter::new(NewNameConfig {
            enabled: true,
            template: "<Date:YYYY-MM-DD>_<Name>".to_string(),
            apply_to: ApplyToOption::Name,
        });

        let ctx = MethodContext {
            original_name: "photo".to_string(),
            original_ext: ".jpg".to_string(),
            ..Default::default()
        };
        let result = method.apply("photo.jpg", &ctx).unwrap();
        assert!(!result.contains('<'), "日期标签不应残留尖括号");
        assert!(result.ends_with("_photo"));
    }

    // ==================== sanitize_filename 函数测试 ====================

    #[test]
    fn test_sanitize_colons_to_hyphens() {
        assert_eq!(sanitize_filename("01:01:01"), "01-01-01");
        assert_eq!(sanitize_filename("61:01"), "61-01");
    }

    #[test]
    fn test_sanitize_angle_brackets() {
        assert_eq!(sanitize_filename("<VidDuration>"), "[VidDuration]");
        assert_eq!(sanitize_filename("<Name>"), "[Name]");
    }

    #[test]
    fn test_sanitize_double_quote_to_single() {
        assert_eq!(sanitize_filename("hello \"world\" photo"), "hello 'world' photo");
    }

    #[test]
    fn test_sanitize_slashes_and_pipe() {
        assert_eq!(sanitize_filename("a/b"), "a-b");
        assert_eq!(sanitize_filename("a\\b"), "a-b");
        assert_eq!(sanitize_filename("a|b"), "a-b");
    }

    #[test]
    fn test_sanitize_question_mark_and_star() {
        assert_eq!(sanitize_filename("hello? world*"), "hello_ world_");
    }

    #[test]
    fn test_sanitize_no_illegal_chars_unchanged() {
        let clean = "vacation_2024-photo.jpg";
        assert_eq!(sanitize_filename(clean), clean);
    }

    #[test]
    fn test_sanitize_mixed_illegal_chars() {
        let result = sanitize_filename("video<01:01:01>done*ok?.jpg");
        assert_eq!(result, "video[01-01-01]done_ok_.jpg");
        assert!(!result.contains('<'));
        assert!(!result.contains('>'));
        assert!(!result.contains(':'));
        assert!(!result.contains('*'));
        assert!(!result.contains('?'));
    }
}
