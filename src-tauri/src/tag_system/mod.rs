// Tag System module - handles dynamic filename generation with tags
// Supports 5 categories: Basic, Timestamp, Sequence, EXIF, ID3

pub mod parser;
pub mod evaluator;

// Re-export main types for convenient access
pub use parser::{TagParser, TagToken};
pub use evaluator::TagEvaluator;
