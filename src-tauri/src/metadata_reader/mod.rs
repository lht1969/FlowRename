// Metadata Reader module - Extracts EXIF and ID3 metadata from files
// Supports JPEG/TIFF EXIF data and MP3 ID3 tags

pub mod exif_reader;
pub mod id3_reader;

use crate::models::metadata::FileMetadata;
use std::path::Path;

pub use exif_reader::ExifReader;
pub use id3_reader::Id3Reader;

/// Unified metadata reader that dispatches to the appropriate
/// specialized reader based on file extension
pub struct MetadataReader;

impl MetadataReader {
    /// Supported image extensions for EXIF extraction
    const IMAGE_EXTENSIONS: &'static [&'static str] = &[
        "jpg", "jpeg", "tif", "tiff", "png", "webp", "heif", "heic",
    ];

    /// Supported audio extensions for ID3 extraction
    const AUDIO_EXTENSIONS: &'static [&'static str] = &[
        "mp3", "wav", "flac", "ogg", "m4a", "aac", "wma",
    ];

    /// Extract metadata from a file based on its extension
    /// Returns a FileMetadata with any available EXIF/ID3 data populated
    pub fn extract(path: &Path) -> FileMetadata {
        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .unwrap_or_default();

        let mut metadata = FileMetadata::default();

        // Extract basic filesystem attributes
        if let Ok(attrs) = std::fs::metadata(path) {
            metadata.is_readonly = attrs.permissions().readonly();
        }

        // Check for hidden attribute on Windows
        #[cfg(target_os = "windows")]
        {
            metadata.is_hidden = Self::is_hidden_windows(path);
        }

        // Extract image EXIF metadata
        if Self::IMAGE_EXTENSIONS.contains(&extension.as_str()) {
            metadata.image = ExifReader::extract(path);
        }

        // Extract audio ID3 metadata
        if Self::AUDIO_EXTENSIONS.contains(&extension.as_str()) {
            metadata.audio = Id3Reader::extract(path);
        }

        metadata
    }

    /// Check if a file has the Windows hidden attribute set
    #[cfg(target_os = "windows")]
    fn is_hidden_windows(path: &Path) -> bool {
        use std::os::windows::fs::MetadataExt;
        if let Ok(attrs) = std::fs::metadata(path) {
            // FILE_ATTRIBUTE_HIDDEN = 0x2
            (attrs.file_attributes() & 0x2) != 0
        } else {
            false
        }
    }

    /// Check if a file extension is supported for metadata extraction
    pub fn is_supported_extension(ext: &str) -> bool {
        let ext_lower = ext.to_lowercase();
        Self::IMAGE_EXTENSIONS.contains(&ext_lower.as_str())
            || Self::AUDIO_EXTENSIONS.contains(&ext_lower.as_str())
    }
}
