// EXIF Reader - Extracts EXIF metadata from image files
// Uses the kamadak-exif crate for parsing

use crate::models::metadata::ImageMetadata;
use std::path::Path;

/// EXIF metadata reader for image files
/// Supports JPEG, TIFF, PNG, WebP, HEIF/HEIC formats
pub struct ExifReader;

impl ExifReader {
    /// Extract EXIF metadata from an image file
    /// Returns None if the file cannot be read or has no EXIF data
    pub fn extract(path: &Path) -> Option<ImageMetadata> {
        let file = std::fs::File::open(path).ok()?;
        let mut bufreader = std::io::BufReader::new(file);
        let exifreader = exif::Reader::new();

        let exif = match exifreader.read_from_container(&mut bufreader) {
            Ok(exif) => exif,
            Err(_) => return None,
        };

        Some(ImageMetadata {
            width: Self::get_u32(&exif, exif::Tag::PixelXDimension)
                .or_else(|| Self::get_u32(&exif, exif::Tag::ImageWidth)),
            height: Self::get_u32(&exif, exif::Tag::PixelYDimension)
                .or_else(|| Self::get_u32(&exif, exif::Tag::ImageLength)),
            make: Self::get_string(&exif, exif::Tag::Make),
            model: Self::get_string(&exif, exif::Tag::Model),
            datetime_original: Self::get_datetime(&exif, exif::Tag::DateTimeOriginal)
                .or_else(|| Self::get_datetime(&exif, exif::Tag::DateTime)),
            iso_speed: Self::get_u32(&exif, exif::Tag::PhotographicSensitivity)
                .or_else(|| Self::get_u32(&exif, exif::Tag::ISOSpeed)),
            f_number: Self::get_rational_as_f32(&exif, exif::Tag::FNumber),
            focal_length: Self::get_rational_as_f32(&exif, exif::Tag::FocalLength),
            exposure_time: Self::get_rational_as_f32(&exif, exif::Tag::ExposureTime),
        })
    }

    /// Get a string value from EXIF data
    fn get_string(exif: &exif::Exif, tag: exif::Tag) -> Option<String> {
        exif.get_field(tag, exif::In::PRIMARY)
            .map(|field| field.display_value().to_string())
    }

    /// Get a u32 value from EXIF data
    fn get_u32(exif: &exif::Exif, tag: exif::Tag) -> Option<u32> {
        exif.get_field(tag, exif::In::PRIMARY)
            .and_then(|field| {
                match &field.value {
                    exif::Value::Short(v) => v.first().copied().map(|v| v as u32),
                    exif::Value::Long(v) => v.first().copied(),
                    _ => None,
                }
            })
    }

    /// Get a DateTime value from EXIF data
    fn get_datetime(exif: &exif::Exif, tag: exif::Tag) -> Option<chrono::DateTime<chrono::Utc>> {
        exif.get_field(tag, exif::In::PRIMARY)
            .and_then(|field| {
                if let exif::Value::Ascii(ref v) = field.value {
                    v.first().and_then(|bytes| {
                        // EXIF datetime format: "YYYY:MM:DD HH:MM:SS"
                        let s = String::from_utf8_lossy(bytes);
                        let naive = chrono::NaiveDateTime::parse_from_str(
                            s.trim_matches('\0').trim(),
                            "%Y:%m:%d %H:%M:%S",
                        ).ok()?;
                        Some(chrono::DateTime::from_naive_utc_and_offset(naive, chrono::Utc))
                    })
                } else {
                    None
                }
            })
    }

    /// Get a Rational value as f32 from EXIF data
    fn get_rational_as_f32(exif: &exif::Exif, tag: exif::Tag) -> Option<f32> {
        exif.get_field(tag, exif::In::PRIMARY)
            .and_then(|field| {
                if let exif::Value::Rational(ref v) = field.value {
                    v.first().map(|r| r.num as f32 / r.denom as f32)
                } else {
                    None
                }
            })
    }
}
