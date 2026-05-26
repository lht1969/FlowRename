use anyhow::{bail, Result};
use chrono::Utc;

use crate::method_engine::MethodContext;

/// Evaluator for resolving tag values based on context
/// Supports 5 categories: Basic, Timestamp, Sequence, EXIF, ID3
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

            // ==================== EXIF TAGS ====================
            "ExifWidth" | "ImgWidth" => self.eval_exif_width(context),
            "ExifHeight" | "ImgHeight" => self.eval_exif_height(context),
            "ExifMake" | "CamMake" => self.eval_exif_make(context),
            "ExifModel" | "CamModel" => self.eval_exif_model(context),
            "ExifDateOrig" | "ShotDate" => self.eval_exif_datetime_original(modifiers, context),
            "ExifISO" | "ISO" => self.eval_exif_iso(context),
            "ExifAperture" | "FNumber" => self.eval_exif_aperture(context),
            "ExifFocalLength" | "FocalLen" => self.eval_exif_focal_length(context),
            "ExifExposureTime" | "ExpTime" => self.eval_exif_exposure_time(context),

            // ==================== ID3 TAGS ====================
            "Id3Title" | "MusicTitle" => self.eval_id3_title(context),
            "Id3Artist" | "MusicArtist" => self.eval_id3_artist(context),
            "Id3Album" | "MusicAlbum" => self.eval_id3_album(context),
            "Id3Year" | "MusicYear" => self.eval_id3_year(context),
            "Id3Track" | "MusicTrack" => self.eval_id3_track(modifiers, context),
            "Id3Genre" | "MusicGenre" => self.eval_id3_genre(context),

            // Unknown tag
            _ => bail!("Unknown tag: '{}'. Supported tags include: Name, Ext, Index, Date, Time, Inc, Cnt, Exif*, Id3*", tag_name),
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
        let padding = modifiers.get(0)
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
}

impl Default for TagEvaluator {
    fn default() -> Self {
        Self::new()
    }
}
