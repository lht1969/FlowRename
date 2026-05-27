// AdRename - Application state stores
// Central state management for the rename workflow

import { writable, derived } from "svelte/store";
import type { FileItem, MethodConfig, FilePreviewItem } from "$lib/types";

/** 当前加载的文件列表 */
export const filesStore = writable<FileItem[]>([]);

/** 当前配置的重命名方法列表（Pipeline） */
export const methodsStore = writable<MethodConfig[]>([]);

/** 预览结果 */
export const previewStore = writable<FilePreviewItem[]>([]);

/** 是否正在加载（扫描/预览/执行） */
export const loadingStore = writable<boolean>(false);

/** 状态栏消息 */
export const statusMessageStore = writable<string>("就绪");

/** 当前扫描的目录路径 */
export const currentDirStore = writable<string>("");

/** 是否递归扫描 */
export const recursiveStore = writable<boolean>(false);

/** 文件扩展名过滤 */
export const extensionFilterStore = writable<string[]>([]);

/** 派生 Store：变更文件数量统计 */
export const statsStore = derived(
  [filesStore, previewStore],
  ([$files, $preview]) => {
    const total = $files.length;
    const changed = $preview.filter((p) => p.isChanged).length;
    const conflicts = $preview.filter((p) => p.hasConflict).length;
    const unchanged = total - changed;

    return { total, changed, conflicts, unchanged };
  },
);

/** 派生 Store：是否有可执行的重命名操作 */
export const canExecuteStore = derived(
  [previewStore, loadingStore],
  ([$preview, $loading]) => {
    return (
      $preview.some((p) => p.isChanged) &&
      !$loading &&
      $preview.every((p) => !p.hasConflict)
    );
  },
);
