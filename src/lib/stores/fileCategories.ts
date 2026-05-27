// AdRename - 文件类别检测 Store
// 根据当前文件列表中文件的扩展名，自动检测是否包含图片/视频/音频文件
// 用于控制标签面板中各类别标签的显示/隐藏

import { writable, derived } from "svelte/store";
import type { FileItem } from "$lib/types";

/** 文件扩展名分类常量 */
const IMAGE_EXTENSIONS = new Set([
  "jpg",
  "jpeg",
  "png",
  "gif",
  "bmp",
  "tif",
  "tiff",
  "webp",
  "heif",
  "heic",
  "svg",
  "ico",
  "raw",
  "cr2",
  "nef",
]);

const VIDEO_EXTENSIONS = new Set([
  "mp4",
  "avi",
  "mkv",
  "mov",
  "wmv",
  "flv",
  "webm",
  "m4v",
  "mpg",
  "mpeg",
  "3gp",
  "ts",
]);

const AUDIO_EXTENSIONS = new Set([
  "mp3",
  "wav",
  "flac",
  "ogg",
  "m4a",
  "aac",
  "wma",
  "opus",
  "aiff",
  "ape",
]);

/** 当前文件列表中是否存在图片文件 */
export const hasImageFiles = writable<boolean>(false);

/** 当前文件列表中是否存在视频文件 */
export const hasVideoFiles = writable<boolean>(false);

/** 当前文件列表中是否存在音频文件 */
export const hasAudioFiles = writable<boolean>(false);

/**
 * 根据文件列表检测并更新各类别文件的存在状态
 * @param files 当前文件列表
 */
export function detectFileCategories(files: FileItem[]) {
  let hasImage = false;
  let hasVideo = false;
  let hasAudio = false;

  for (const file of files) {
    const ext = file.originalExt.toLowerCase().replace(".", "");
    if (IMAGE_EXTENSIONS.has(ext)) {
      hasImage = true;
    }
    if (VIDEO_EXTENSIONS.has(ext)) {
      hasVideo = true;
    }
    if (AUDIO_EXTENSIONS.has(ext)) {
      hasAudio = true;
    }
    // 提前退出：三类都已检测到
    if (hasImage && hasVideo && hasAudio) {
      break;
    }
  }

  hasImageFiles.set(hasImage);
  hasVideoFiles.set(hasVideo);
  hasAudioFiles.set(hasAudio);
}
