// Tag System module - handles dynamic filename generation with tags
// Supports 4 categories: Basic/Timestamp/Sequence, Image (Img), Video (Vid), Audio (Aud)
// 旧标签名（Exif*/Id3*）保留为别名以确保向后兼容

pub mod parser;
pub mod evaluator;

// Re-export main types for convenient access
pub use parser::{TagParser, TagToken};
pub use evaluator::TagEvaluator;
