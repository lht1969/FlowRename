// AdRename - TypeScript type definitions for Tauri IPC
// These types mirror the Rust structs for type-safe communication

/** File item returned from scan_directory command */
export interface FileItem {
  id: string;
  originalPath: string;
  originalName: string;
  originalExt: string;
  previewName: string | null;
  fileSize: number;
  createdTime: string;
  modifiedTime: string;
  accessedTime: string;
  metadata: FileMetadata | null;
  status: FileStatus;
}

/** File processing status */
export type FileStatus =
  | "Pending"
  | "PreviewReady"
  | "Processing"
  | "Success"
  | { Failed: string }
  | "Conflict"
  | "Skipped";

/** Extended file metadata (EXIF/ID3/Video) */
export interface FileMetadata {
  image?: ImageMetadata;
  audio?: AudioMetadata;
  video?: VideoMetadata;
}

/** Image EXIF metadata */
export interface ImageMetadata {
  width?: number;
  height?: number;
  make?: string;
  model?: string;
  datetimeOriginal?: string;
  isoSpeed?: number;
  fNumber?: number;
  focalLength?: number;
  exposureTime?: number;
}

/** Audio ID3 metadata */
export interface AudioMetadata {
  title?: string;
  artist?: string;
  album?: string;
  year?: number;
  trackNumber?: number;
  genre?: string;
  discNumber?: number;
  totalDiscs?: number;
  duration?: number;
  bitrate?: number;
}

/** Video metadata (MP4, MKV, AVI 等) */
export interface VideoMetadata {
  width?: number;
  height?: number;
  frameRate?: number;
  durationSecs?: number;
  title?: string;
  genre?: string;
  creationDate?: string;
  codec?: string;
  bitRate?: number;
}

/** Request to scan a directory */
export interface ScanRequest {
  directoryPath: string;
  recursive: boolean;
  fileExtensions: string[];
}

/** Request to scan specific files */
export interface ScanFilesRequest {
  filePaths: string[];
}

/** Response from scan_directory command */
export interface ScanResponse {
  files: FileItem[];
  totalCount: number;
  directoriesScanned: number;
  elapsedMs: number;
  error: string | null;
}

/** Method configuration types */
export type MethodConfig =
  | { Replace: ReplaceConfig }
  | { Add: AddConfig }
  | { Remove: RemoveConfig }
  | { NewCase: NewCaseConfig }
  | { NewName: NewNameConfig }
  | { List: ListConfig }
  | { Move: MoveConfig }
  | { Trim: TrimConfig }
  | { Renumber: RenumberConfig }
  | { Timestamp: TimestampConfig };

export interface ReplaceConfig {
  enabled: boolean;
  find: string;
  replaceWith: string;
  occurrence: "All" | "First" | "Last" | { Custom: number };
  caseSensitive: boolean;
  useRegex: boolean;
  applyTo: "Name" | "Extension" | "Both";
}

export interface AddConfig {
  enabled: boolean;
  text: string;
  position: "Start" | "End" | { Custom: number };
  customIndex: number | null;
  backwards: boolean;
  applyTo: "Name" | "Extension" | "Both";
}

export interface RemoveConfig {
  enabled: boolean;
  count: number;
  position: "Start" | "End";
  applyTo: "Name" | "Extension" | "Both";
}

export interface NewCaseConfig {
  enabled: boolean;
  newCase: "Lower" | "Upper" | "Title" | "Sentence" | "Inverted";
  location: "All" | "First";
  applyTo: "Name" | "Extension" | "Both";
}

export interface NewNameConfig {
  enabled: boolean;
  template: string;
  applyTo: "Name" | "Extension" | "Both";
}

/** List method - rename files from a list of names */
export interface ListConfig {
  enabled: boolean;
  names: string[];
  overflowBehavior: "KeepOriginal" | "Skip" | "Cycle";
  applyTo: "Name" | "Extension" | "Both";
}

/** Move method - move characters within filename */
export interface MoveConfig {
  enabled: boolean;
  fromStart: number;
  count: number;
  toPosition: number;
  applyTo: "Name" | "Extension" | "Both";
}

/** Trim method - trim characters from edges */
export interface TrimConfig {
  enabled: boolean;
  trimStart: string;
  trimEnd: string;
  trimWhitespace: boolean;
  applyTo: "Name" | "Extension" | "Both";
}

/** Renumber method - add sequential numbers */
export interface RenumberConfig {
  enabled: boolean;
  start: number;
  step: number;
  padding: number;
  position: "Prefix" | "Suffix" | "Replace";
  separator: string;
  applyTo: "Name" | "Extension" | "Both";
}

/** Timestamp method - rename based on file timestamps */
export interface TimestampConfig {
  enabled: boolean;
  source: "Created" | "Modified" | "Accessed" | "ImgDate" | "ImgTime" | "VidDate" | "VidTime" | "AudDate" | "AudTime";
  format: string;
  applyTo: "Name" | "Extension" | "Both";
}

/** Request to preview rename operations */
export interface PreviewRequest {
  methods: MethodConfig[];
  template?: string;
}

/** Single file preview result */
export interface FilePreviewItem {
  originalPath: string;
  originalName: string;
  newName: string;
  isChanged: boolean;
  hasConflict: boolean;
}

/** Response from preview_rename command */
export interface PreviewResponse {
  files: FilePreviewItem[];
  totalCount: number;
  changedCount: number;
  conflictCount: number;
  elapsedMs: number;
  error: string | null;
}

/** Single rename item for execution */
export interface RenameItem {
  currentPath: string;
  newName: string;
}

/** Request to execute rename operations */
export interface ExecuteRenameRequest {
  renameItems: RenameItem[];
  createUndoHistory: boolean;
}

/** Details about a failed rename */
export interface RenameError {
  originalPath: string;
  attemptedName: string;
  reason: string;
}

/** Response from execute_rename command */
export interface ExecuteResponse {
  successCount: number;
  failedCount: number;
  errors: RenameError[];
  elapsedMs: number;
  error: string | null;
}

/** Response from undo_last_rename command */
export interface UndoResponse {
  success: boolean;
  restoredCount: number;
  error: string | null;
}

/** Response from get_undo_status command */
export interface UndoStatusResponse {
  hasHistory: boolean;
  entryCount: number;
  totalOperations: number;
  lastEntryTime: string | null;
}
