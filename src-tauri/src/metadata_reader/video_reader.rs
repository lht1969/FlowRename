// Video Reader - 从视频文件中提取元数据
// 使用 mp4parse 解析 MP4 容器格式（MP4/M4V/MOV）

use crate::models::video_metadata::VideoMetadata;
use std::path::Path;

/// 视频元数据读取器
/// 支持 MP4 容器格式（MP4, M4V, MOV）
pub struct VideoReader;

impl VideoReader {
    /// 支持通过 mp4parse 解析的扩展名
    const MP4PARSE_EXTENSIONS: &'static [&'static str] = &[
        "mp4", "m4v", "mov", "3gp",
    ];

    /// 从 MP4 二进制数据中提取 mvhd box 的 creation_time
    /// MP4 时间戳使用 1904-01-01 作为 epoch
    fn extract_mp4_creation_time(data: &[u8]) -> Option<chrono::DateTime<chrono::Utc>> {
        const MVHD_BOX_TYPE: &[u8; 4] = b"mvhd";
        const MOOV_BOX_TYPE: &[u8; 4] = b"moov";
        const MP4_EPOCH_OFFSET: i64 = -2082844800; // 1904-01-01 到 1970-01-01 的秒数差

        let mut offset = 0;
        while offset < data.len() - 8 {
            let box_size = u32::from_be_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]]) as usize;
            let box_type = &data[offset + 4..offset + 8];

            if box_size == 0 || box_size < 8 {
                break;
            }

            if box_type == MOOV_BOX_TYPE {
                let moov_start = offset + 8;
                let moov_end = (offset + box_size).min(data.len());
                let moov_data = &data[moov_start..moov_end];

                let mut moov_offset = 0;
                while moov_offset < moov_data.len() - 8 {
                    let child_size = u32::from_be_bytes([moov_data[moov_offset], moov_data[moov_offset + 1], moov_data[moov_offset + 2], moov_data[moov_offset + 3]]) as usize;
                    let child_type = &moov_data[moov_offset + 4..moov_offset + 8];

                    if child_size == 0 || child_size < 8 {
                        break;
                    }

                    if child_type == MVHD_BOX_TYPE {
                        let mvhd_data = &moov_data[moov_offset + 8..moov_offset + child_size];
                        if mvhd_data.len() >= 16 {
                            let version = mvhd_data[0];

                            let creation_secs = if version == 1 {
                                if mvhd_data.len() < 24 {
                                    break;
                                }
                                i64::from_be_bytes([
                                    mvhd_data[8], mvhd_data[9], mvhd_data[10], mvhd_data[11],
                                    mvhd_data[12], mvhd_data[13], mvhd_data[14], mvhd_data[15]
                                ])
                            } else {
                                if mvhd_data.len() < 16 {
                                    break;
                                }
                                u32::from_be_bytes([
                                    mvhd_data[8], mvhd_data[9], mvhd_data[10], mvhd_data[11]
                                ]) as i64
                            };

                            const BEIJING_OFFSET_SECS: i64 = 8 * 3600;
                            let local_secs = creation_secs - BEIJING_OFFSET_SECS;
                            let unix_secs = local_secs.checked_add(MP4_EPOCH_OFFSET)?;
                            if let Some(dt) = chrono::DateTime::from_timestamp(unix_secs, 0) {
                                return Some(dt);
                            }
                        }
                        break;
                    }

                    moov_offset += child_size;
                }
                break;
            }

            offset += box_size;
        }

        None
    }

    /// 从视频文件提取元数据
    /// 根据文件扩展名选择合适的解析器
    /// 返回 None 表示无法读取或无元数据
    pub fn extract(path: &Path) -> Option<VideoMetadata> {
        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .unwrap_or_default();

        match extension.as_str() {
            ext if Self::MP4PARSE_EXTENSIONS.contains(&ext) => Self::extract_mp4(path),
            // 其他格式（mkv, avi, wmv 等）暂不支持，后续可集成 ffmpeg
            _ => None,
        }
    }

    /// 使用 mp4parse 从 MP4 容器中提取元数据
    fn extract_mp4(path: &Path) -> Option<VideoMetadata> {
        let data = std::fs::read(path).ok()?;
        let mut cursor = std::io::Cursor::new(&data);

        let context = match mp4parse::read_mp4(&mut cursor) {
            Ok(ctx) => ctx,
            Err(_) => return None,
        };

        let mut metadata = VideoMetadata::default();

        // 提取视频创建日期 (从 mvhd box 的 creation_time)
        metadata.creation_date = Self::extract_mp4_creation_time(&data);

        // 从全局 timescale 计算总时长
        if let Some(ref global_ts) = context.timescale {
            let global_timescale = global_ts.0 as f32;
            if global_timescale > 0.0 {
                // 遍历所有轨道，找到最长的视频轨道时长
                for track in &context.tracks {
                    if let Some(ref tkhd) = track.tkhd {
                        let track_duration_secs = tkhd.duration as f32 / global_timescale;
                        if track_duration_secs > metadata.duration_secs.unwrap_or(0.0) {
                            metadata.duration_secs = Some(track_duration_secs);
                        }
                    }
                }
            }
        }

        // 从视频轨道中提取宽高和编码信息
        for track in &context.tracks {
            if let Some(ref stsd) = track.stsd {
                for entry in &stsd.descriptions {
                    if let mp4parse::SampleEntry::Video(ref video_entry) = entry {
                        // 提取宽高（VideoSampleEntry 的 width/height 是实际像素值）
                        if metadata.width.is_none() {
                            metadata.width = Some(video_entry.width as u32);
                        }
                        if metadata.height.is_none() {
                            metadata.height = Some(video_entry.height as u32);
                        }

                        // 提取编码格式
                        if metadata.codec.is_none() {
                            metadata.codec = Some(Self::codec_type_to_string(video_entry.codec_type));
                        }

                        // 只取第一个视频轨道的信息
                        break;
                    }
                }
            }
        }

        // 仅当至少获取到一项有效数据时返回
        if metadata.width.is_some()
            || metadata.height.is_some()
            || metadata.duration_secs.is_some()
        {
            Some(metadata)
        } else {
            None
        }
    }

    /// 将 mp4parse CodecType 转换为可读字符串
    fn codec_type_to_string(codec_type: mp4parse::CodecType) -> String {
        match codec_type {
            mp4parse::CodecType::H264 => "H.264".to_string(),
            mp4parse::CodecType::MP4V => "MPEG-4".to_string(),
            mp4parse::CodecType::AV1 => "AV1".to_string(),
            mp4parse::CodecType::VP9 => "VP9".to_string(),
            mp4parse::CodecType::VP8 => "VP8".to_string(),
            mp4parse::CodecType::H263 => "H.263".to_string(),
            _ => "Unknown".to_string(),
        }
    }
}
