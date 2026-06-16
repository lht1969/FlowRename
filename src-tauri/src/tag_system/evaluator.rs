use anyhow::{bail, Result};
use chrono::{DateTime, Datelike, Utc};

use crate::method_engine::MethodContext;

/// Evaluator for resolving tag values based on context
/// Supports 4 categories: Basic/Timestamp/Sequence, Image (Img), Video (Vid), Audio (Aud)
/// 旧标签名（Exif*/Id3*）保留为别名以确保向后兼容
pub struct TagEvaluator;

impl TagEvaluator {
    /// Create a new TagEvaluator instance
    pub fn new() -> Self {
        Self
    }

    /// Evaluate a complete template string, replacing all tags with their values
    /// 
    /// # Arguments
    /// * `template` - The template string containing tags (e.g., "<Date:YYYY>_<Inc:3>")
    /// * `context` - Execution context with file metadata and batch info
    /// 
    /// # Returns
    /// The fully evaluated string with all tags replaced by their computed values.
    /// 
    /// # Errors
    /// Returns error if any tag evaluation fails or if the template is invalid.
    pub fn evaluate_template(&self, template: &str, context: &MethodContext) -> Result<String> {
        use super::parser::TagParser;
        let parser = TagParser::new();
        let tokens = parser.parse(template);

        // If no tags found, return template as-is
        if tokens.is_empty() {
            return Ok(template.to_string());
        }

        let mut result = template.to_string();

        // Replace each tag with its value (process in reverse to maintain positions)
        for token in tokens.iter().rev() {
            let value = self.evaluate_tag(&token.tag_name, &token.modifiers, context)?;
            result = result.replace(&token.original_text, &value);
        }

        Ok(result)
    }

    /// Evaluate a single tag and return its value
    /// 
    /// # Arguments
    /// * `tag_name` - Name of the tag to evaluate (e.g., "Date", "Inc", "Name")
    /// * `modifiers` - Optional modifiers/parameters for the tag
    /// * `context` - Execution context with file metadata and batch info
    /// 
    /// # Returns
    /// The evaluated string value of the tag.
    /// 
    /// # Errors
    /// Returns error if:
    /// - Tag name is unknown/not supported
    /// - Modifiers are invalid for this tag type
    /// - Required context data is missing
    pub fn evaluate_tag(
        &self,
        tag_name: &str,
        modifiers: &[String],
        context: &MethodContext,
    ) -> Result<String> {
        match tag_name {
            // ==================== BASIC TAGS ====================
            "Name" => self.eval_basic_name(context),
            "Ext" => self.eval_basic_ext(context),
            "Index" => self.eval_basic_index(modifiers, context),

            // ==================== TIMESTAMP TAGS ====================
            "Date" => self.eval_timestamp_date(modifiers),
            "Time" => self.eval_timestamp_time(modifiers),
            "Now" | "DateTime" => self.eval_timestamp_datetime(modifiers),

            // ==================== SEQUENCE TAGS ====================
            "Inc" => self.eval_sequence_inc(modifiers, context),
            "Cnt" | "Count" => self.eval_sequence_count(modifiers, context),

            // ==================== IMAGE TAGS (Img prefix) ====================
            "ImgWidth" | "ExifWidth" => self.eval_exif_width(context),
            "ImgHeight" | "ExifHeight" => self.eval_exif_height(context),
            "ImgMake" | "ExifMake" | "CamMake" => self.eval_exif_make(context),
            "ImgModel" | "ExifModel" | "CamModel" => self.eval_exif_model(context),
            "ImgDate" | "ExifDateOrig" | "ShotDate" => self.eval_exif_datetime_original(modifiers, context),
            "ImgISO" | "ExifISO" | "ISO" => self.eval_exif_iso(context),
            "ImgAperture" | "ExifAperture" | "FNumber" => self.eval_exif_aperture(context),
            "ImgFocal" | "ExifFocalLength" | "FocalLen" => self.eval_exif_focal_length(context),
            "ImgExposure" | "ExifExposureTime" | "ExpTime" => self.eval_exif_exposure_time(context),
            "ImgTime" => self.eval_exif_time(modifiers, context),

            // ==================== VIDEO TAGS (Vid prefix) ====================
            "VidWidth" => self.eval_video_width(context),
            "VidHeight" => self.eval_video_height(context),
            "VidFrameRate" | "VidFps" => self.eval_video_frame_rate(context),
            "VidDuration" => self.eval_video_duration(modifiers, context),
            "VidDurationSec" => self.eval_video_duration_secs(context),
            "VidTitle" => self.eval_video_title(context),
            "VidGenre" => self.eval_video_genre(context),
            "VidDate" | "VidDateOrig" => self.eval_video_date(modifiers, context),
            "VidCodec" => self.eval_video_codec(context),
            "VidBitRate" | "VidBitrate" => self.eval_video_bitrate(context),
            "VidTime" => self.eval_video_time(modifiers, context),

            // ==================== AUDIO TAGS (Aud prefix) ====================
            "AudTitle" | "Id3Title" | "MusicTitle" => self.eval_id3_title(context),
            "AudArtist" | "Id3Artist" | "MusicArtist" => self.eval_id3_artist(context),
            "AudAlbum" | "Id3Album" | "MusicAlbum" => self.eval_id3_album(context),
            "AudYear" | "Id3Year" | "MusicYear" => self.eval_id3_year(context),
            "AudTrack" | "Id3Track" | "MusicTrack" => self.eval_id3_track(modifiers, context),
            "AudGenre" | "Id3Genre" | "MusicGenre" => self.eval_id3_genre(context),
            "AudDuration" => self.eval_audio_duration(modifiers, context),
            "AudDurationSec" => self.eval_audio_duration_secs(context),
            "AudBitRate" | "AudBitrate" => self.eval_audio_bitrate(context),
            "AudDisc" => self.eval_audio_disc(modifiers, context),
            "AudDate" => self.eval_audio_date(modifiers, context),
            "AudTime" => self.eval_audio_time(modifiers, context),

            // Unknown tag
            _ => bail!("Unknown tag: '{}'. Supported tags: Name, Ext, Index, Date, Time, Inc, Cnt, Img*, Vid*, Aud*", tag_name),
        }
    }

    // ==================== BASIC TAG IMPLEMENTATIONS ====================

    fn eval_basic_name(&self, context: &MethodContext) -> Result<String> {
        Ok(context.original_name.clone())
    }

    fn eval_basic_ext(&self, context: &MethodContext) -> Result<String> {
        Ok(context.original_ext.clone())
    }

    fn eval_basic_index(&self, _modifiers: &[String], context: &MethodContext) -> Result<String> {
        // Return 1-based index
        Ok((context.file_index + 1).to_string())
    }

    // ==================== TIMESTAMP TAG IMPLEMENTATIONS ====================

    fn eval_timestamp_date(&self, modifiers: &[String]) -> Result<String> {
        let format = modifiers.first()
            .map(|s| s.as_str())
            .unwrap_or("YYYYMMDD");  // Default format
        
        let now = Utc::now();
        Ok(now.format(self.convert_chrono_format(format)).to_string())
    }

    fn eval_timestamp_time(&self, modifiers: &[String]) -> Result<String> {
        let format = modifiers.first()
            .map(|s| s.as_str())
            .unwrap_or("HHmmss");  // Default format
        
        let now = Utc::now();
        Ok(now.format(self.convert_chrono_format(format)).to_string())
    }

    fn eval_timestamp_datetime(&self, modifiers: &[String]) -> Result<String> {
        let format = modifiers.first()
            .map(|s| s.as_str())
            .unwrap_or("YYYYMMDD_HHmmss");  // Default format
        
        let now = Utc::now();
        Ok(now.format(self.convert_chrono_format(format)).to_string())
    }

    fn parse_legacy_date(&self, legacy: &str) -> Option<chrono::DateTime<Utc>> {
        if legacy.len() == 4 {
            let day: u32 = legacy[..2].parse().ok()?;
            let month: u32 = legacy[2..4].parse().ok()?;
            let year = Utc::now().date_naive().year();
            return chrono::NaiveDate::from_ymd_opt(year, month, day)
                .and_then(|d| d.and_hms_opt(0, 0, 0))
                .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc));
        }
        None
    }

    /// Convert our custom format specifiers to chrono's format
    fn convert_chrono_format<'a>(&self, format: &'a str) -> &'a str {
        match format {
            // Date formats
            "YYYY" => "%Y",
            "YY" => "%y",
            "MM" => "%m",
            "DD" => "%d",
            "YYYYMMDD" => "%Y%m%d",
            "YYYY-MM-DD" => "%Y-%m-%d",
            "YYYY/MM/DD" => "%Y/%m/%d",
            "YYMMDD" => "%y%m%d",
            "DDMMYYYY" => "%d%m%Y",
            "MMDDYYYY" => "%m%d%Y",
            
            // Time formats
            "HH" => "%H",
            "mm" => "%M",
            "ss" => "%S",
            "HHmmss" => "%H%M%S",
            "HH-mm-ss" => "%H-%M-%S",
            "HH:mm:ss" => "%H:%M:%S",
            
            // Combined datetime formats
            "YYYYMMDD_HHmmss" => "%Y%m%d_%H%M%S",
            "YYYY-MM-DD_HH-mm-ss" => "%Y-%m-%d_%H-%M-%S",
            
            _ => format,  // Pass through as-is if we don't recognize it
        }
    }

    // ==================== SEQUENCE TAG IMPLEMENTATIONS ====================

    fn eval_sequence_inc(&self, modifiers: &[String], context: &MethodContext) -> Result<String> {
        // Modifiers: [padding_digits] or [padding_digits, start_value]
        let padding = modifiers.first()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(1);  // Default: no padding
        
        let start = modifiers.get(1)
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(1);  // Default: start from 1
        
        let value = start + context.file_index;
        
        Ok(format!("{:0width$}", value, width = padding))
    }

    fn eval_sequence_count(&self, modifiers: &[String], context: &MethodContext) -> Result<String> {
        // Total count of files in batch
        let padding = modifiers.first()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(1);
        
        Ok(format!("{:0width$}", context.total_files, width = padding))
    }

    // ==================== EXIF TAG IMPLEMENTATIONS ====================

    fn eval_exif_width(&self, context: &MethodContext) -> Result<String> {
        match &context.file_metadata {
            Some(meta) => match &meta.image {
                Some(img) => match img.width {
                    Some(w) => Ok(w.to_string()),
                    None => bail!("Image width metadata not available"),
                },
                None => bail!("No image metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    fn eval_exif_height(&self, context: &MethodContext) -> Result<String> {
        match &context.file_metadata {
            Some(meta) => match &meta.image {
                Some(img) => match img.height {
                    Some(h) => Ok(h.to_string()),
                    None => bail!("Image height metadata not available"),
                },
                None => bail!("No image metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    fn eval_exif_make(&self, context: &MethodContext) -> Result<String> {
        match &context.file_metadata {
            Some(meta) => match &meta.image {
                Some(img) => match &img.make {
                    Some(make) => Ok(make.clone()),
                    None => bail!("Camera make metadata not available"),
                },
                None => bail!("No image metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    fn eval_exif_model(&self, context: &MethodContext) -> Result<String> {
        match &context.file_metadata {
            Some(meta) => match &meta.image {
                Some(img) => match &img.model {
                    Some(model) => Ok(model.clone()),
                    None => bail!("Camera model metadata not available"),
                },
                None => bail!("No image metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    fn eval_exif_datetime_original(&self, modifiers: &[String], context: &MethodContext) -> Result<String> {
        let format = modifiers.first()
            .map(|s| s.as_str())
            .unwrap_or("YYYYMMDD_HHmmss");
        
        match &context.file_metadata {
            Some(meta) => match &meta.image {
                Some(img) => match &img.datetime_original {
                    Some(dt) => Ok(dt.format(self.convert_chrono_format(format)).to_string()),
                    None => bail!("Original date/time metadata not available"),
                },
                None => bail!("No image metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    fn eval_exif_iso(&self, context: &MethodContext) -> Result<String> {
        match &context.file_metadata {
            Some(meta) => match &meta.image {
                Some(img) => match img.iso_speed {
                    Some(iso) => Ok(iso.to_string()),
                    None => bail!("ISO speed metadata not available"),
                },
                None => bail!("No image metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    fn eval_exif_aperture(&self, context: &MethodContext) -> Result<String> {
        match &context.file_metadata {
            Some(meta) => match &meta.image {
                Some(img) => match img.f_number {
                    Some(f) => Ok(format!("{:.1}", f)),
                    None => bail!("Aperture (f-number) metadata not available"),
                },
                None => bail!("No image metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    fn eval_exif_focal_length(&self, context: &MethodContext) -> Result<String> {
        match &context.file_metadata {
            Some(meta) => match &meta.image {
                Some(img) => match img.focal_length {
                    Some(fl) => Ok(format!("{:.0}mm", fl)),
                    None => bail!("Focal length metadata not available"),
                },
                None => bail!("No image metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    fn eval_exif_exposure_time(&self, context: &MethodContext) -> Result<String> {
        match &context.file_metadata {
            Some(meta) => match &meta.image {
                Some(img) => match img.exposure_time {
                    Some(et) => {
                        if et >= 1.0 {
                            Ok(format!("{:.0}s", et))
                        } else {
                            Ok(format!("1/{}", (1.0 / et).round() as u32))
                        }
                    }
                    None => bail!("Exposure time metadata not available"),
                },
                None => bail!("No image metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    fn eval_exif_time(&self, modifiers: &[String], context: &MethodContext) -> Result<String> {
        let format = modifiers.first()
            .map(|s| s.as_str())
            .unwrap_or("HHmmss");

        match &context.file_metadata {
            Some(meta) => match &meta.image {
                Some(img) => match img.datetime_original {
                    Some(dt) => Ok(dt.format(self.convert_chrono_format(format)).to_string()),
                    None => bail!("Original date/time metadata not available"),
                },
                None => bail!("No image metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    // ==================== ID3 TAG IMPLEMENTATIONS ====================

    fn eval_id3_title(&self, context: &MethodContext) -> Result<String> {
        match &context.file_metadata {
            Some(meta) => match &meta.audio {
                Some(audio) => match &audio.title {
                    Some(title) => Ok(title.clone()),
                    None => bail!("ID3 title tag not available"),
                },
                None => bail!("No audio metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    fn eval_id3_artist(&self, context: &MethodContext) -> Result<String> {
        match &context.file_metadata {
            Some(meta) => match &meta.audio {
                Some(audio) => match &audio.artist {
                    Some(artist) => Ok(artist.clone()),
                    None => bail!("ID3 artist tag not available"),
                },
                None => bail!("No audio metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    fn eval_id3_album(&self, context: &MethodContext) -> Result<String> {
        match &context.file_metadata {
            Some(meta) => match &meta.audio {
                Some(audio) => match &audio.album {
                    Some(album) => Ok(album.clone()),
                    None => bail!("ID3 album tag not available"),
                },
                None => bail!("No audio metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    fn eval_id3_year(&self, context: &MethodContext) -> Result<String> {
        match &context.file_metadata {
            Some(meta) => match &meta.audio {
                Some(audio) => match audio.year {
                    Some(year) => Ok(year.to_string()),
                    None => bail!("ID3 year tag not available"),
                },
                None => bail!("No audio metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    fn eval_id3_track(&self, modifiers: &[String], context: &MethodContext) -> Result<String> {
        let padding = modifiers.first()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(2);  // Default: 2-digit track numbers
        
        match &context.file_metadata {
            Some(meta) => match &meta.audio {
                Some(audio) => match audio.track_number {
                    Some(track) => Ok(format!("{:0width$}", track, width = padding)),
                    None => bail!("ID3 track number not available"),
                },
                None => bail!("No audio metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    fn eval_id3_genre(&self, context: &MethodContext) -> Result<String> {
        match &context.file_metadata {
            Some(meta) => match &meta.audio {
                Some(audio) => match &audio.genre {
                    Some(genre) => Ok(genre.clone()),
                    None => bail!("ID3 genre tag not available"),
                },
                None => bail!("No audio metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    fn eval_audio_date(&self, modifiers: &[String], context: &MethodContext) -> Result<String> {
        let format = modifiers.first()
            .map(|s| s.as_str())
            .unwrap_or("YYYYMMDD");

        match &context.file_metadata {
            Some(meta) => match &meta.audio {
                Some(audio) => {
                    if let Some(dt) = &audio.recording_date {
                        return Ok(dt.format(self.convert_chrono_format(format)).to_string());
                    }
                    if let Some(legacy) = &audio.legacy_date {
                        if let Some(parsed) = self.parse_legacy_date(legacy) {
                            return Ok(parsed.format(self.convert_chrono_format(format)).to_string());
                        }
                    }
                    bail!("Audio recording date not available (TDRC/TDAT)")
                },
                None => bail!("No audio metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    fn eval_audio_time(&self, modifiers: &[String], context: &MethodContext) -> Result<String> {
        let format = modifiers.first()
            .map(|s| s.as_str())
            .unwrap_or("HHmmss");

        match &context.file_metadata {
            Some(meta) => match &meta.audio {
                Some(audio) => match &audio.recording_date {
                    Some(dt) => Ok(dt.format(self.convert_chrono_format(format)).to_string()),
                    None => bail!("Audio recording date not available"),
                },
                None => bail!("No audio metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    // ==================== VIDEO TAG IMPLEMENTATIONS ====================

    fn eval_video_width(&self, context: &MethodContext) -> Result<String> {
        match &context.file_metadata {
            Some(meta) => match &meta.video {
                Some(vid) => match vid.width {
                    Some(w) => Ok(w.to_string()),
                    None => bail!("Video width metadata not available"),
                },
                None => bail!("No video metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    fn eval_video_height(&self, context: &MethodContext) -> Result<String> {
        match &context.file_metadata {
            Some(meta) => match &meta.video {
                Some(vid) => match vid.height {
                    Some(h) => Ok(h.to_string()),
                    None => bail!("Video height metadata not available"),
                },
                None => bail!("No video metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    fn eval_video_frame_rate(&self, context: &MethodContext) -> Result<String> {
        match &context.file_metadata {
            Some(meta) => match &meta.video {
                Some(vid) => match vid.frame_rate {
                    Some(fps) => Ok(format!("{:.2}", fps)),
                    None => bail!("Video frame rate metadata not available"),
                },
                None => bail!("No video metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    fn eval_video_duration(&self, modifiers: &[String], context: &MethodContext) -> Result<String> {
        let format = modifiers.first()
            .map(|s| s.as_str())
            .unwrap_or("HH-MM-SS");

        match &context.file_metadata {
            Some(meta) => match &meta.video {
                Some(vid) => match vid.duration_secs {
                    Some(secs) => Ok(Self::format_duration(secs, format)),
                    None => bail!("Video duration metadata not available"),
                },
                None => bail!("No video metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    fn eval_video_duration_secs(&self, context: &MethodContext) -> Result<String> {
        match &context.file_metadata {
            Some(meta) => match &meta.video {
                Some(vid) => match vid.duration_secs {
                    Some(secs) => Ok(format!("{:.0}", secs)),
                    None => bail!("Video duration metadata not available"),
                },
                None => bail!("No video metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    fn eval_video_title(&self, context: &MethodContext) -> Result<String> {
        match &context.file_metadata {
            Some(meta) => match &meta.video {
                Some(vid) => match &vid.title {
                    Some(title) => Ok(title.clone()),
                    None => bail!("Video title metadata not available"),
                },
                None => bail!("No video metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    fn eval_video_genre(&self, context: &MethodContext) -> Result<String> {
        match &context.file_metadata {
            Some(meta) => match &meta.video {
                Some(vid) => match &vid.genre {
                    Some(genre) => Ok(genre.clone()),
                    None => bail!("Video genre metadata not available"),
                },
                None => bail!("No video metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    fn eval_video_date(&self, modifiers: &[String], context: &MethodContext) -> Result<String> {
        let format = modifiers.first()
            .map(|s| s.as_str())
            .unwrap_or("YYYYMMDD_HHmmss");

        match &context.file_metadata {
            Some(meta) => match &meta.video {
                Some(vid) => match &vid.creation_date {
                    Some(dt) => Ok(dt.format(self.convert_chrono_format(format)).to_string()),
                    None => bail!("Video creation date metadata not available"),
                },
                None => bail!("No video metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    fn eval_video_codec(&self, context: &MethodContext) -> Result<String> {
        match &context.file_metadata {
            Some(meta) => match &meta.video {
                Some(vid) => match &vid.codec {
                    Some(codec) => Ok(codec.clone()),
                    None => bail!("Video codec metadata not available"),
                },
                None => bail!("No video metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    fn eval_video_bitrate(&self, context: &MethodContext) -> Result<String> {
        match &context.file_metadata {
            Some(meta) => match &meta.video {
                Some(vid) => match vid.bit_rate {
                    Some(br) => Ok(br.to_string()),
                    None => bail!("Video bitrate metadata not available"),
                },
                None => bail!("No video metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    fn eval_video_time(&self, modifiers: &[String], context: &MethodContext) -> Result<String> {
        let format = modifiers.first()
            .map(|s| s.as_str())
            .unwrap_or("HHmmss");

        match &context.file_metadata {
            Some(meta) => match &meta.video {
                Some(vid) => match vid.creation_date {
                    Some(dt) => Ok(dt.format(self.convert_chrono_format(format)).to_string()),
                    None => bail!("Video creation date metadata not available"),
                },
                None => bail!("No video metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    // ==================== AUDIO EXTENDED TAG IMPLEMENTATIONS ====================

    fn eval_audio_duration(&self, modifiers: &[String], context: &MethodContext) -> Result<String> {
        let format = modifiers.first()
            .map(|s| s.as_str())
            .unwrap_or("MM-SS");

        match &context.file_metadata {
            Some(meta) => match &meta.audio {
                Some(aud) => match aud.duration {
                    Some(secs) => Ok(Self::format_duration(secs as f32, format)),
                    None => bail!("Audio duration metadata not available"),
                },
                None => bail!("No audio metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    fn eval_audio_duration_secs(&self, context: &MethodContext) -> Result<String> {
        match &context.file_metadata {
            Some(meta) => match &meta.audio {
                Some(aud) => match aud.duration {
                    Some(secs) => Ok(secs.to_string()),
                    None => bail!("Audio duration metadata not available"),
                },
                None => bail!("No audio metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    fn eval_audio_bitrate(&self, context: &MethodContext) -> Result<String> {
        match &context.file_metadata {
            Some(meta) => match &meta.audio {
                Some(aud) => match aud.bitrate {
                    Some(br) => Ok(br.to_string()),
                    None => bail!("Audio bitrate metadata not available"),
                },
                None => bail!("No audio metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    fn eval_audio_disc(&self, modifiers: &[String], context: &MethodContext) -> Result<String> {
        let padding = modifiers.first()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(1);

        match &context.file_metadata {
            Some(meta) => match &meta.audio {
                Some(aud) => match aud.disc_number {
                    Some(disc) => Ok(format!("{:0width$}", disc, width = padding)),
                    None => bail!("Audio disc number not available"),
                },
                None => bail!("No audio metadata available"),
            },
            None => bail!("No file metadata available. Ensure MetadataReader is enabled."),
        }
    }

    /// 将秒数格式化为时长字符串
    fn format_duration(secs: f32, format: &str) -> String {
        let total_secs = secs as u32;
        let hours = total_secs / 3600;
        let minutes = (total_secs % 3600) / 60;
        let seconds = total_secs % 60;

        match format {
            "HHMMSS" => format!("{:02}{:02}{:02}", hours, minutes, seconds),
            "HH:MM:SS" => format!("{:02}:{:02}:{:02}", hours, minutes, seconds),
            "HH-MM-SS" => format!("{:02}-{:02}-{:02}", hours, minutes, seconds),
            "MM:SS" => {
                let total_mins = total_secs / 60;
                format!("{:02}:{:02}", total_mins, seconds)
            }
            "MM-SS" => {
                let total_mins = total_secs / 60;
                format!("{:02}-{:02}", total_mins, seconds)
            }
            _ => {
                // 默认可读格式
                if hours > 0 {
                    format!("{}h {}m {}s", hours, minutes, seconds)
                } else if minutes > 0 {
                    format!("{}m {}s", minutes, seconds)
                } else {
                    format!("{}s", seconds)
                }
            }
        }
    }
}

impl Default for TagEvaluator {
    fn default() -> Self {
        Self::new()
    }
}
