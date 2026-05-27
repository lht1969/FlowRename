use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::metadata_reader::MetadataReader;
use crate::models::{FileItem, FileStatus};

/// File Manager - handles file system operations
/// Responsible for loading, scanning, and filtering files from the filesystem
pub struct FileManager;

impl FileManager {
    /// Load files from paths (supports both files and directories)
    ///
    /// # Arguments
    /// * `paths` - List of file or directory paths to load
    /// * `recursive` - Whether to scan directories recursively (true) or only top-level (false)
    /// * `extensions_filter` - Optional filter to only include files with certain extensions (e.g., vec!["jpg", "png"])
    ///                        Extensions should be lowercase without dots. Case-insensitive comparison.
    ///
    /// # Returns
    /// A `Result` containing a vector of `FileItem` objects representing all found and valid files.
    /// Returns an error if no valid paths are provided or all paths fail to load.
    ///
    /// # Examples
    /// ```ignore
    /// // Load a single file
    /// let files = FileManager::load_files(vec![path.to_path_buf()], false, None);
    ///
    /// // Load a directory recursively with image filter
    /// let filter = vec!["jpg".to_string(), "png".to_string()];
    /// let images = FileManager::load_files(vec![dir.to_path_buf()], true, Some(filter));
    /// ```
    pub fn load_files(
        paths: Vec<PathBuf>,
        recursive: bool,
        extensions_filter: Option<Vec<String>>,
    ) -> Result<Vec<FileItem>> {
        // Validate that at least one path was provided
        if paths.is_empty() {
            anyhow::bail!("No paths provided for file loading");
        }

        let mut file_items = Vec::new();

        for path in paths {
            match Self::process_path(&path, recursive, &extensions_filter) {
                Ok(mut items) => file_items.append(&mut items),
                Err(e) => {
                    log::warn!("Failed to process path {:?}: {}", path, e);
                    // Continue processing other paths instead of failing entirely
                    // This allows partial success when some paths are invalid
                }
            }
        }

        // Validate that we found at least one file
        if file_items.is_empty() {
            anyhow::bail!("No valid files found in provided paths");
        }

        Ok(file_items)
    }

    /// Process a single path (file or directory)
    fn process_path(
        path: &PathBuf,
        recursive: bool,
        extensions_filter: &Option<Vec<String>>,
    ) -> Result<Vec<FileItem>> {
        let mut file_items = Vec::new();

        if path.is_file() {
            // Single file - create FileItem directly
            let item = Self::create_file_item(path)?;
            file_items.push(item);
        } else if path.is_dir() {
            // Directory - scan contents
            if recursive {
                // Recursive mode: use WalkDir for deep traversal
                // Note: We do NOT use follow_links(true) to avoid infinite loops
                // from circular symlinks/reparse points (common on Windows).
                // Max depth is also capped for safety.
                let walker = WalkDir::new(path)
                    .max_depth(100)
                    .into_iter()
                    .filter_map(|e| {
                        if let Err(err) = &e {
                            log::warn!("WalkDir error (skipping): {}", err);
                        }
                        e.ok()
                    })
                    .filter(|e| e.file_type().is_file());

                for entry in walker {
                    // Apply extension filter if provided
                    if let Some(ref filter) = extensions_filter {
                        if !Self::matches_extension_filter(entry.path(), filter) {
                            continue;
                        }
                    }

                    match Self::create_file_item(entry.path()) {
                        Ok(item) => file_items.push(item),
                        Err(e) => {
                            log::warn!("Skipping file {:?} due to error: {}", entry.path(), e);
                        }
                    }
                }
            } else {
                // Non-recursive mode: only scan top-level files using fs::read_dir
                let entries = fs::read_dir(path)?;

                for entry_result in entries {
                    match entry_result {
                        Ok(entry) => {
                            let entry_path = entry.path();

                            // Skip directories in non-recursive mode
                            if !entry_path.is_file() {
                                continue;
                            }

                            // Apply extension filter if provided
                            if let Some(ref filter) = extensions_filter {
                                if !Self::matches_extension_filter(&entry_path, filter) {
                                    continue;
                                }
                            }

                            match Self::create_file_item(&entry_path) {
                                Ok(item) => file_items.push(item),
                                Err(e) => {
                                    log::warn!(
                                        "Skipping file {:?} due to error: {}",
                                        entry_path,
                                        e
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            log::warn!("Failed to read directory entry: {}", e);
                        }
                    }
                }
            }
        } else {
            // Path doesn't exist or is not a regular file/directory
            anyhow::bail!(
                "Invalid path: {:?}. Path does not exist or is not accessible.",
                path
            );
        }

        Ok(file_items)
    }

    /// Check if a file's extension matches the filter criteria
    ///
    /// # Arguments
    /// * `file_path` - Path to the file being checked
    /// * `filter` - List of allowed extensions (lowercase, without dots)
    ///
    /// # Returns
    /// true if the file matches the filter (or has no extension when filtering), false otherwise
    fn matches_extension_filter(file_path: &std::path::Path, filter: &[String]) -> bool {
        // Get the file extension
        let ext = match file_path.extension() {
            Some(e) => e.to_string_lossy().to_lowercase(),
            None => {
                // Files without extension don't match any extension filter
                return false;
            }
        };

        // Check if this extension is in our allowed list
        filter
            .iter()
            .any(|allowed_ext| allowed_ext.to_lowercase() == ext)
    }

    /// Create a single FileItem from a filesystem path
    /// Extracts metadata (size, timestamps) and creates the data structure
    ///
    /// # Arguments
    /// * `path` - Absolute path to the file
    ///
    /// # Returns
    /// A `Result` containing the created `FileItem`, or an error if:
    /// - The path doesn't exist
    /// - Metadata cannot be read
    /// - Filename cannot be parsed
    fn create_file_item(path: &std::path::Path) -> Result<FileItem> {
        // Validate that the path exists
        if !path.exists() {
            anyhow::bail!("Path does not exist: {:?}", path);
        }

        // Read filesystem metadata (size, timestamps, attributes)
        let metadata = fs::metadata(path)
            .with_context(|| format!("Failed to read filesystem metadata for {:?}", path))?;

        // Extract filename components
        let name_stem = path
            .file_stem()
            .ok_or_else(|| {
                anyhow::anyhow!("Invalid filename: cannot extract name stem from {:?}", path)
            })?
            .to_string_lossy()
            .to_string();

        let extension = path
            .extension()
            .map(|e| format!(".{}", e.to_string_lossy()))
            .unwrap_or_default();

        // Convert filesystem times to DateTime<Utc> for consistent handling
        let created_time = metadata
            .created()
            .map(Into::into)
            .unwrap_or_else(|_| chrono::Utc::now());

        let modified_time = metadata
            .modified()
            .map(Into::into)
            .unwrap_or_else(|_| chrono::Utc::now());

        let accessed_time = metadata
            .accessed()
            .map(Into::into)
            .unwrap_or_else(|_| chrono::Utc::now());

        // Extract extended metadata (EXIF/ID3) for supported file types
        let file_metadata =
            if MetadataReader::is_supported_extension(&extension.trim_start_matches('.')) {
                Some(MetadataReader::extract(path))
            } else {
                None
            };

        Ok(FileItem {
            id: uuid::Uuid::new_v4(),
            original_path: path.to_path_buf(),
            original_name: name_stem,
            original_ext: extension,
            preview_name: None,
            file_size: metadata.len(),
            created_time,
            modified_time,
            accessed_time,
            metadata: file_metadata,
            status: FileStatus::Pending,
        })
    }
}

/// Sorts a list of FileItem objects by the specified field and direction.
///
/// Uses natural sorting (StrCmpLogicalW style) for filename comparisons, where:
///   - Digit sequences are compared as numbers: "file2" < "file10"
///   - ASCII letters are compared case-insensitively: "a" == "A"
///   - Non-ASCII characters (e.g. Chinese) are compared by Unicode code point
///
/// This is more compatible with Windows Explorer's sorting than standard lexicographic sort.
pub fn sort_file_items(items: &mut Vec<FileItem>, field: &str, desc: bool) {
    match field {
        "name" => {
            items.sort_by(|a, b| {
                let cmp = compare_filename_natural(&a.original_name, &b.original_name);
                if desc { cmp.reverse() } else { cmp }
            });
        }
        "size" => {
            items.sort_by(|a, b| {
                let cmp = a.file_size.cmp(&b.file_size);
                if desc { cmp.reverse() } else { cmp }
            });
        }
        "modified" => {
            items.sort_by(|a, b| {
                let cmp = a.modified_time.cmp(&b.modified_time);
                if desc { cmp.reverse() } else { cmp }
            });
        }
        _ => {
            items.sort_by(|a, b| {
                let cmp = compare_filename_natural(&a.original_name, &b.original_name);
                if desc { cmp.reverse() } else { cmp }
            });
        }
    }
}

/// 检查字符是否为汉字 (CJK Unified Ideographs)
fn is_cjk_char(c: char) -> bool {
    matches!(c, '\u{4E00}'..='\u{9FFF}')
        || matches!(c, '\u{3400}'..='\u{4DBF}')
        || matches!(c, '\u{20000}'..='\u{2A6DF}')
        || matches!(c, '\u{2A700}'..='\u{2B73F}')
        || matches!(c, '\u{2B740}'..='\u{2B81F}')
        || matches!(c, '\u{2B820}'..='\u{2CEAF}')
        || matches!(c, '\u{2F800}'..='\u{2FA1F}')
}

/// 获取单个汉字的拼音，如果字符不是汉字则返回 None
fn get_char_pinyin(c: char) -> Option<String> {
    if is_cjk_char(c) {
        let args = pinyin::Args::new();
        let result = pinyin::lazy_pinyin(&c.to_string(), &args);
        if !result.is_empty() {
            return Some(result[0].to_string());
        }
    }
    None
}

/// 自然排序文件名比较函数 (StrCmpLogicalW 风格, 支持拼音排序)
fn compare_filename_natural(a: &str, b: &str) -> std::cmp::Ordering {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let mut a_idx = 0;
    let mut b_idx = 0;

    while a_idx < a_chars.len() && b_idx < b_chars.len() {
        let a_ch = a_chars[a_idx];
        let b_ch = b_chars[b_idx];

        if a_ch.is_ascii_digit() && b_ch.is_ascii_digit() {
            let a_num = extract_number(&a_chars, &mut a_idx);
            let b_num = extract_number(&b_chars, &mut b_idx);
            match a_num.cmp(&b_num) {
                std::cmp::Ordering::Equal => {}
                ord => return ord,
            }
        } else {
            let a_pinyin = get_char_pinyin(a_ch);
            let b_pinyin = get_char_pinyin(b_ch);

            match (a_pinyin, b_pinyin) {
                (Some(ap), Some(bp)) => {
                    match ap.cmp(&bp) {
                        std::cmp::Ordering::Equal => {
                            a_idx += 1;
                            b_idx += 1;
                        }
                        ord => return ord,
                    }
                }
                (Some(_), None) => {
                    let a_lower = a_ch.to_ascii_lowercase();
                    let b_lower = b_ch.to_ascii_lowercase();
                    match a_lower.cmp(&b_lower) {
                        std::cmp::Ordering::Equal => {
                            a_idx += 1;
                            b_idx += 1;
                        }
                        ord => return ord,
                    }
                }
                (None, Some(_)) => {
                    let a_lower = a_ch.to_ascii_lowercase();
                    let b_lower = b_ch.to_ascii_lowercase();
                    match a_lower.cmp(&b_lower) {
                        std::cmp::Ordering::Equal => {
                            a_idx += 1;
                            b_idx += 1;
                        }
                        ord => return ord,
                    }
                }
                (None, None) => {
                    let a_lower = a_ch.to_ascii_lowercase();
                    let b_lower = b_ch.to_ascii_lowercase();
                    match a_lower.cmp(&b_lower) {
                        std::cmp::Ordering::Equal => {
                            a_idx += 1;
                            b_idx += 1;
                        }
                        ord => return ord,
                    }
                }
            }
        }
    }

    a_chars.len().cmp(&b_chars.len())
}

fn extract_number(chars: &[char], start_idx: &mut usize) -> u64 {
    let mut num_str = String::new();
    while *start_idx < chars.len() && chars[*start_idx].is_ascii_digit() {
        num_str.push(chars[*start_idx]);
        *start_idx += 1;
    }
    num_str.parse().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_natural_sort_chinese() {
        let mut names = vec!["安次得胜口马氏家谱序", "新建 DOCX 文档", "AI编程工具OpenCode"];
        names.sort_by(|a, b| compare_filename_natural(a, b));
        assert_eq!(names[0], "AI编程工具OpenCode");
        assert_eq!(names[1], "安次得胜口马氏家谱序");
        assert_eq!(names[2], "新建 DOCX 文档");
    }

    #[test]
    fn test_natural_sort_numbers() {
        let mut names = vec!["file10", "file2", "file1"];
        names.sort_by(|a, b| compare_filename_natural(a, b));
        assert_eq!(names[0], "file1");
        assert_eq!(names[1], "file2");
        assert_eq!(names[2], "file10");
    }

    #[test]
    fn test_natural_sort_mixed() {
        let mut names = vec![
            "本项目的结果-AI编程工具OpenCode",
            "新建 DOCX 文档",
            "安次得胜口马氏家谱序",
        ];
        names.sort_by(|a, b| compare_filename_natural(a, b));
        // 拼音: 安(an) < 本(ben) < 新(xin)
        assert_eq!(names[0], "安次得胜口马氏家谱序");
        assert_eq!(names[1], "本项目的结果-AI编程工具OpenCode");
        assert_eq!(names[2], "新建 DOCX 文档");
    }
}
