use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 视频文件元数据（MP4, MKV, AVI, MOV 等）
/// 包含视频流信息和容器级元数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VideoMetadata {
    /// 视频宽度（像素）
    pub width: Option<u32>,

    /// 视频高度（像素）
    pub height: Option<u32>,

    /// 帧率（fps）
    pub frame_rate: Option<f32>,

    /// 视频时长（秒）
    pub duration_secs: Option<f32>,

    /// 视频标题（来自容器元数据）
    pub title: Option<String>,

    /// 视频流派/类型
    pub genre: Option<String>,

    /// 视频创建日期
    pub creation_date: Option<DateTime<Utc>>,

    /// 视频编码格式（如 "H.264", "HEVC"）
    pub codec: Option<String>,

    /// 视频比特率（kbps）
    pub bit_rate: Option<u32>,
}
