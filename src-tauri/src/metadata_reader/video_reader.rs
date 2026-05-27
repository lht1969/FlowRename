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
