// AdRename - Tauri IPC command wrappers
// Provides type-safe access to all backend Tauri commands

import { invoke } from "@tauri-apps/api/core";
import type {
  ScanRequest,
  ScanFilesRequest,
  ScanResponse,
  PreviewRequest,
  PreviewResponse,
  ExecuteRenameRequest,
  ExecuteResponse,
  UndoResponse,
  UndoStatusResponse,
} from "./types";

/** 扫描目录并加载文件列表 */
export async function scanDirectory(
  request: ScanRequest,
): Promise<ScanResponse> {
  return invoke<ScanResponse>("scan_directory", { request });
}

/** 扫描选中的文件列表 */
export async function scanFiles(
  request: ScanFilesRequest,
): Promise<ScanResponse> {
  return invoke<ScanResponse>("scan_files", { request });
}

/** 清空当前文件列表 */
export async function clearFiles(): Promise<void> {
  return invoke("clear_files");
}

/** 预览重命名操作结果 */
export async function previewRename(
  request: PreviewRequest,
): Promise<PreviewResponse> {
  return invoke<PreviewResponse>("preview_rename", { request });
}

/** 执行实际文件重命名 */
export async function executeRename(
  request: ExecuteRenameRequest,
): Promise<ExecuteResponse> {
  return invoke<ExecuteResponse>("execute_rename", { request });
}

/** 撤销上次重命名操作 */
export async function undoLastRename(): Promise<UndoResponse> {
  return invoke<UndoResponse>("undo_last_rename");
}

/** 查询撤销历史状态 */
export async function getUndoStatus(): Promise<UndoStatusResponse> {
  return invoke<UndoStatusResponse>("get_undo_status");
}
