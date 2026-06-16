// ID3 Reader - Extracts ID3 metadata from audio files
// Uses the id3 crate for MP3 files, with basic support for other formats

use crate::models::metadata::AudioMetadata;
use id3::TagLike;
use std::path::Path;

/// ID3 metadata reader for audio files
/// Primarily supports MP3 with ID3v2 tags
pub struct Id3Reader;

impl Id3Reader {
    /// Extract ID3 metadata from an audio file
    /// Returns None if the file cannot be read or has no ID3 data
    pub fn extract(path: &Path) -> Option<AudioMetadata> {
        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .unwrap_or_default();

        match extension.as_str() {
            "mp3" => Self::extract_mp3(path),
            _ => None,
        }
    }

    /// Extract metadata from an MP3 file using the id3 crate
    fn extract_mp3(path: &Path) -> Option<AudioMetadata> {
        let tag = match id3::Tag::read_from_path(path) {
            Ok(tag) => tag,
            Err(_) => {
                // File might not have ID3 tags, try to create a default metadata
                return Self::extract_basic_audio_info(path);
            }
        };

        Some(AudioMetadata {
            title: tag.title().map(|s: &str| s.to_string()),
            artist: tag.artist().map(|s: &str| s.to_string()),
            album: tag.album().map(|s: &str| s.to_string()),
            year: tag.year().map(|y: i32| y as u16),
            track_number: tag.track().map(|t| t as u16),
            genre: tag.genre().map(|s: &str| s.to_string()),
            duration: Self::get_duration_mp3(path),
            bitrate: Self::get_bitrate_mp3(path),
            ..Default::default()
        })
    }

    /// Extract basic audio info when no ID3 tags are present
    fn extract_basic_audio_info(path: &Path) -> Option<AudioMetadata> {
        let duration = Self::get_duration_mp3(path);
        let bitrate = Self::get_bitrate_mp3(path);

        if duration.is_some() || bitrate.is_some() {
            Some(AudioMetadata { duration, bitrate, ..Default::default() })
        } else {
            None
        }
    }

    /// Get duration of an MP3 file in seconds
    /// Uses a simple estimation based on file size and bitrate
    fn get_duration_mp3(path: &Path) -> Option<u32> {
        let file_size = std::fs::metadata(path).ok()?.len() as u64;

        // Rough estimation: assume average bitrate of 320kbps
        // Duration (seconds) = file_size * 8 / bitrate
        let estimated_duration = (file_size * 8) / (320 * 1000);

        if estimated_duration > 0 && estimated_duration < 36000 {
            Some(estimated_duration as u32)
        } else {
            None
        }
    }

    /// Get bitrate of an MP3 file in kbps
    /// Uses a simple estimation based on file size
    fn get_bitrate_mp3(path: &Path) -> Option<u32> {
        let file_size = std::fs::metadata(path).ok()?.len() as u64;
        let duration = Self::get_duration_mp3(path)?;

        if duration > 0 {
            let bitrate = (file_size * 8) / (duration as u64 * 1000);
            Some(bitrate as u32)
        } else {
            None
        }
    }
}
