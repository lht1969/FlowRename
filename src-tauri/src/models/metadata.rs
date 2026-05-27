use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::video_metadata::VideoMetadata;

/// Container for all types of file metadata
/// Holds both basic filesystem attributes and extended media-specific metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileMetadata {
    /// Basic filesystem attributes
    pub is_readonly: bool,
    pub is_hidden: bool,
    pub is_system: bool,

    /// Image-specific EXIF metadata (if applicable)
    /// None for non-image files or when not yet extracted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<ImageMetadata>,

    /// Audio-specific ID3 metadata (if applicable)
    /// None for non-audio files or when not yet extracted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<AudioMetadata>,

    /// 视频文件元数据（MP4, MKV, AVI 等）
    /// None 表示非视频文件或尚未提取
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<VideoMetadata>,
}

impl Default for FileMetadata {
    fn default() -> Self {
        Self {
            is_readonly: false,
            is_hidden: false,
            is_system: false,
            image: None,
            audio: None,
            video: None,
        }
    }
}

/// EXIF metadata extracted from image files (JPEG, TIFF, etc.)
/// Uses standard EXIF field names for compatibility
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ImageMetadata {
    /// Image width in pixels
    pub width: Option<u32>,

    /// Image height in pixels
    pub height: Option<u32>,

    /// Camera manufacturer (e.g., "Canon", "Nikon")
    pub make: Option<String>,

    /// Camera model (e.g., "EOS R5", "D850")
    pub model: Option<String>,

    /// Date and time the photo was taken (from EXIF DateTimeOriginal)
    pub datetime_original: Option<DateTime<Utc>>,

    /// ISO sensitivity value
    pub iso_speed: Option<u32>,

    /// Aperture f-number (e.g., 2.8 for f/2.8)
    pub f_number: Option<f32>,

    /// Focal length in millimeters
    pub focal_length: Option<f32>,

    /// Exposure time in seconds (e.g., 0.005 for 1/200s)
    pub exposure_time: Option<f32>,
}

/// ID3 metadata extracted from audio files (MP3, etc.)
/// Supports ID3v2 tags with common fields
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AudioMetadata {
    /// Song title
    pub title: Option<String>,

    /// Artist/performer name
    pub artist: Option<String>,

    /// Album name
    pub album: Option<String>,

    /// Release year
    pub year: Option<u16>,

    /// Track number on the album
    pub track_number: Option<u16>,

    /// Music genre (e.g., "Pop", "Rock", "Classical")
    pub genre: Option<String>,

    /// 碟片号（专辑中的第几张碟）
    pub disc_number: Option<u16>,

    /// 总碟片数
    pub total_discs: Option<u16>,

    /// Duration in seconds
    pub duration: Option<u32>,

    /// Bitrate in kbps (e.g., 320 for 320kbps MP3)
    pub bitrate: Option<u32>,

    /// 录制日期（ID3v2.4 TDRC 字段，优先使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_date: Option<chrono::DateTime<Utc>>,

    /// 旧版录制日期（ID3v2.2/2.3 TDAT/TIME 字段，降级使用）
    /// 格式通常为 DDMM 或 DDMMHHmm
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_date: Option<String>,
}
