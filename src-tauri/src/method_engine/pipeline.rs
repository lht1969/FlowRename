use anyhow::{bail, Result};

use super::traits::{Method, MethodContext};

/// Pipeline that chains multiple methods together for sequential execution
/// Methods are executed in the order they were added (first-in, first-executed)
pub struct Pipeline {
    /// Ordered list of methods to execute
    methods: Vec<Box<dyn Method>>,
}

impl Pipeline {
    /// Create a new empty pipeline with no methods
    pub fn new() -> Self {
        Self { 
            methods: Vec::new() 
        }
    }

    /// Add a method to the end of the pipeline
    /// The method will be executed after all previously added methods
    pub fn add_method(&mut self, method: Box<dyn Method>) {
        self.methods.push(method);
    }

    /// Get the number of methods currently in the pipeline
    pub fn len(&self) -> usize {
        self.methods.len()
    }

    /// Check if the pipeline has no methods
    pub fn is_empty(&self) -> bool {
        self.methods.is_empty()
    }

    /// Execute all methods in sequence on the input string
    /// 
    /// # Arguments
    /// * `input` - The original filename to transform
    /// * `context` - Execution context with file metadata and batch info
    /// 
    /// # Returns
    /// The fully transformed string after all methods have been applied,
    /// or an error if any method fails.
    /// 
    /// # Behavior
    /// - Empty input returns an error (cannot process empty strings)
    /// - Each method's output becomes the next method's input
    /// - If any method produces empty output, returns an error
    /// - Processing stops immediately on first error
    /// - Methods execute in the order they were added to the pipeline
    /// 
    /// # Example
    /// ```ignore
    /// // If pipeline has: [Prefix("IMG_"), Suffix("_v2")]
    /// // And input is "photo.jpg"
    /// // Step 1: Prefix → "IMG_photo.jpg"
    /// // Step 2: Suffix → "IMG_photo_v2.jpg"
    /// ```
    pub fn execute(&self, input: &str, context: &MethodContext) -> Result<String> {
        // Validate input is not empty
        if input.is_empty() {
            bail!("Cannot process empty input string");
        }

        let mut result = input.to_string();

        // Execute each method in sequence
        for (index, method) in self.methods.iter().enumerate() {
            // Log progress for debugging and monitoring
            log::debug!(
                "Executing method #{} ({}) on '{}' (length: {})",
                index + 1,
                method.name(),
                &result[..result.len().min(50)],
                result.len()
            );

            // Apply the transformation
            match method.apply(&result, context) {
                Ok(new_result) => {
                    // Validate the result is not empty
                    if new_result.is_empty() {
                        bail!(
                            "Method #{} ('{}') produced empty output. \
                             If intentional removal is needed, use Remove method instead.",
                            index + 1,
                            method.name()
                        );
                    }
                    
                    // Update result for next iteration
                    result = new_result;
                    
                    log::trace!(
                        "Method #{} ('{}') completed successfully",
                        index + 1,
                        method.name()
                    );
                }
                Err(e) => {
                    // Log and return the error immediately
                    log::error!(
                        "Method #{} ('{}') failed: {}",
                        index + 1,
                        method.name(),
                        e
                    );
                    bail!(
                        "Method #{} ('{}') failed: {}",
                        index + 1,
                        method.name(),
                        e
                    );
                }
            }
        }

        Ok(result)
    }

    /// Clear all methods from the pipeline
    /// Resets the pipeline to empty state
    pub fn clear(&mut self) {
        self.methods.clear();
    }

    /// Remove method at specified index (0-based)
    /// 
    /// # Arguments
    /// * `index` - Position of method to remove
    /// 
    /// # Returns
    /// Some(method) if index was valid, None if out of bounds
    pub fn remove_at(&mut self, index: usize) -> Option<Box<dyn Method>> {
        if index < self.methods.len() {
            Some(self.methods.remove(index))
        } else {
            None
        }
    }

    /// Get reference to method at specified index (for inspection)
    /// 
    /// # Arguments
    /// * `index` - Position of method to retrieve
    /// 
    /// # Returns
    /// Some(reference) if index was valid, None if out of bounds
    pub fn get_method(&self, index: usize) -> Option<&dyn Method> {
        self.methods.get(index).map(|m| m.as_ref())
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}

/// Windows 文件名中不允许的字符及安全替换字符
/// 
/// Windows 禁止的字符: < > : " / \ | ? *
/// 替换规则:
///   ':' → '-'   (时间/时长分隔符的自然替代)
///   '<' → '['   (保持结构可读)
///   '>' → ']'   (保持结构可读)
///   '"' → '\''  (保持引号语义)
///   '/', '\\', '|' → '-'
///   '?'  → '_'
///   '*'  → '_'
pub fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            ':' => '-',
            '<' => '[',
            '>' => ']',
            '"' => '\'',
            '/' | '\\' | '|' => '-',
            '?' | '*' => '_',
            _ => c,
        })
        .collect()
}
