// Method Engine module - handles renaming methods and pipeline execution
// This module provides the core transformation logic for filename renaming

pub mod traits;
pub mod pipeline;

// Re-export main types for convenient access
pub use traits::{Method, MethodContext};
pub use pipeline::Pipeline;
