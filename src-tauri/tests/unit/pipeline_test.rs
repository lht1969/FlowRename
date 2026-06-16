#[cfg(test)]
mod tests {
    use flowrename::method_engine::{Method, MethodContext, Pipeline};
    use flowrename::models::ApplyToOption;
    use anyhow::Result;

    // Simple test method that prefixes text (for testing pipeline)
    #[derive(Debug)]
    struct PrefixMethod {
        prefix: String,
    }

    impl Method for PrefixMethod {
        fn name(&self) -> &str { "TestPrefix" }
        
        fn method_type(&self) -> flowrename::models::MethodType {
            // Placeholder - will be defined when we implement actual methods
            unimplemented!()
        }
        
        fn is_enabled(&self) -> bool { true }
        
        fn apply(&self, input: &str, _context: &MethodContext) -> Result<String> {
            Ok(format!("{}{}", self.prefix, input))
        }
        
        fn validate(&self) -> Result<()> {
            Ok(())
        }
        
        fn to_config(&self) -> flowrename::models::MethodConfig {
            // Placeholder
            unimplemented!()
        }

        fn apply_to(&self) -> ApplyToOption {
            ApplyToOption::Both
        }
    }

    #[test]
    fn test_empty_pipeline_returns_input() {
        let pipeline = Pipeline::new();
        let ctx = MethodContext::default();
        let result = pipeline.execute("hello.txt", &ctx);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello.txt");
    }

    #[test]
    fn test_single_method_pipeline() {
        let mut pipeline = Pipeline::new();
        pipeline.add_method(Box::new(PrefixMethod { 
            prefix: "IMG_".into() 
        }));
        
        let ctx = MethodContext::default();
        let result = pipeline.execute("photo.jpg", &ctx);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "IMG_photo.jpg");
    }

    #[test]
    fn test_multiple_methods_execute_in_order() {
        let mut pipeline = Pipeline::new();
        pipeline.add_method(Box::new(PrefixMethod { 
            prefix: "A_".into() 
        }));
        pipeline.add_method(Box::new(PrefixMethod { 
            prefix: "B_".into() 
        }));
        
        let ctx = MethodContext::default();
        let result = pipeline.execute("file.txt", &ctx);
        
        // Methods execute in order they were added
        // First "A_" is added → "A_file.txt"
        // Then "B_" is added → "B_A_file.txt"
        assert_eq!(result.unwrap(), "B_A_file.txt");
    }

    #[test]
    fn test_pipeline_length_tracking() {
        let mut pipeline = Pipeline::new();
        
        assert_eq!(pipeline.len(), 0);
        assert!(pipeline.is_empty());
        
        pipeline.add_method(Box::new(PrefixMethod { 
            prefix: "X_".into() 
        }));
        
        assert_eq!(pipeline.len(), 1);
        assert!(!pipeline.is_empty());
    }

    #[test]
    fn test_pipeline_clear_and_remove() {
        let mut pipeline = Pipeline::new();
        pipeline.add_method(Box::new(PrefixMethod { 
            prefix: "1_".into() 
        }));
        pipeline.add_method(Box::new(PrefixMethod { 
            prefix: "2_".into() 
        }));
        
        assert_eq!(pipeline.len(), 2);
        
        // Remove first method
        let removed = pipeline.remove_at(0);
        assert!(removed.is_some());
        assert_eq!(pipeline.len(), 1);
        
        // Clear all methods
        pipeline.clear();
        assert!(pipeline.is_empty());
    }

    #[test]
    fn test_method_context_default_values() {
        let ctx = MethodContext::default();
        
        assert_eq!(ctx.file_index, 0);
        assert_eq!(ctx.total_files, 1);
        assert!(ctx.file_metadata.is_none());
        assert_eq!(ctx.original_name, "");
        assert_eq!(ctx.original_ext, "");
    }
}
