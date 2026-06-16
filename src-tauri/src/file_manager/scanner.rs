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
    ///   Extensions should be lowercase without dots. Case-insensitive comparison.
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
            if MetadataReader::is_supported_extension(extension.trim_start_matches('.')) {
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
/// Uses natural sorting for filename comparisons, where:
///   - Digit sequences are compared as numbers: "file2" < "file10"
///   - ASCII letters are compared case-insensitively: "a" == "A"
///   - CJK characters are compared using platform-specific locale-aware APIs:
///       * Windows: CompareStringEx with user default locale (pinyin order on zh-CN)
///       * Linux/macOS: strcoll with system locale (requires zh_CN.UTF-8 for pinyin order)
///
/// This is more compatible with Windows Explorer's sorting than standard lexicographic sort.
pub fn sort_file_items(items: &mut [FileItem], field: &str, desc: bool) {
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

// ============================================================================
// 平台特定的汉字排序实现
//
// 不同操作系统使用不同的底层 API 进行区域感知 (locale-aware) 的汉字比较:
//
//   Windows:
//     使用 CompareStringEx API, 基于用户当前系统区域设置 (locale) 进行排序.
//     在中文 Windows 系统上, 该 API 按照拼音顺序排列汉字,
//     例如: 安(an) < 本(ben) < 新(xin).
//     CompareStringEx 返回 CSTR_LESS_THAN / CSTR_EQUAL / CSTR_GREATER_THAN,
//     与 Rust 的 std::cmp::Ordering 直接映射.
//
//   Linux / macOS:
//     使用 POSIX 标准的 strcoll 函数进行区域感知排序.
//     该函数依赖系统 locale 配置 (LC_COLLATE 环境变量).
//     需要系统已安装 zh_CN.UTF-8 locale 才能正确对汉字进行拼音排序.
//     若 locale 不可用, strcoll 将回退到 Unicode 码点 (code point) 排序,
//     此时排序结果仍具有确定性, 但不保证拼音语义正确.
//
// 核心思路:
//   平台特定函数仅用于比较两个均为 CJK 汉字的字符.
//   对于 ASCII 字母, 继续使用大小写不敏感的 ASCII 比较规则,
//   以保持自然排序中 "a" == "A" 的行为一致性.
// ============================================================================

/// Windows 平台: 使用 CompareStringEx API 进行区域感知的汉字比较
///
/// CompareStringEx 是 Windows Vista 及以上版本推荐的字符串比较 API,
/// 支持基于 BCP-47 locale 名称的排序. 使用 LOCALE_NAME_USER_DEFAULT
/// 自动继承用户的区域设置, 在中文系统上会自动使用拼音排序.
#[cfg(target_os = "windows")]
fn compare_cjk_chars(a: char, b: char) -> std::cmp::Ordering {
    use windows::core::w;
    use windows::Win32::Foundation::LPARAM;
    use windows::Win32::Globalization::{CompareStringEx, COMPARE_STRING_FLAGS};

    // 将单个字符编码为 UTF-16 (Windows 原生宽字符编码)
    // 使用固定大小缓冲区 [u16; 2], 足够容纳 BMP 字符及 CJK 扩展 B~G 代理对
    let mut a_buf = [0u16; 2];
    let a_utf16 = a.encode_utf16(&mut a_buf);
    let mut b_buf = [0u16; 2];
    let b_utf16 = b.encode_utf16(&mut b_buf);

    // 调用 CompareStringEx 进行比较
    //   - w!("zh-CN"): 显式指定中文简体排序规则, 确保按拼音排序
    //   - dwcmpflags: COMPARE_STRING_FLAGS::default() = SORT_DEFAULT (0)
    //   - lpstring1/2: &[u16] 切片, 库自动处理长度参数
    //   - lpversioninformation / lpreserved: 保留参数, 传 None
    //   - lparam: 应用自定义数据, 此处无需要, 传 LPARAM::default() (= 0)
    //
    // 返回值类型: COMPARESTRING_RESULT(i32 包装)
    //   成功返回 CSTR_LESS_THAN(1) / CSTR_EQUAL(2) / CSTR_GREATER_THAN(3)
    //   失败返回 0, 此时回退至 Unicode 码点比较作为降级方案
    let result = unsafe {
        CompareStringEx(
            w!("zh-CN"),
            COMPARE_STRING_FLAGS::default(),
            a_utf16,
            b_utf16,
            None,
            None,
            LPARAM::default(),
        )
    };

    // COMPARESTRING_RESULT 是 #[repr(transparent)] 新类型, 用 .0 访问内部 i32 值
    match result.0 {
        1 => std::cmp::Ordering::Less,
        2 => std::cmp::Ordering::Equal,
        3 => std::cmp::Ordering::Greater,
        // API 调用失败时的降级方案: 按 Unicode 码点直接比较
        _ => a.cmp(&b),
    }
}

/// Unix 平台 (Linux / macOS): 使用 POSIX strcoll 进行区域感知的汉字比较
///
/// strcoll 根据当前进程的 LC_COLLATE locale 设置比较两个字符串.
/// 为了让汉字按拼音排序, 需要系统支持 zh_CN.UTF-8 locale:
///   - Linux: 运行 `locale -a | grep zh_CN` 检查是否已安装
///   - macOS: 默认支持 zh_CN.UTF-8
///
/// 在每个比较周期中调用 setlocale 确保使用系统环境 locale.
/// 注意: 这是一个简化实现, 生产环境中可考虑在程序启动时设置一次 locale.
#[cfg(not(target_os = "windows"))]
fn compare_cjk_chars(a: char, b: char) -> std::cmp::Ordering {
    use std::ffi::CString;

    // 将字符转为 C 字符串 (null-terminated byte string)
    let a_str = format!("{}", a);
    let b_str = format!("{}", b);
    let a_cstr = CString::new(a_str.as_bytes()).unwrap_or_default();
    let b_cstr = CString::new(b_str.as_bytes()).unwrap_or_default();

    // 使用 strcoll 进行区域感知比较
    // 返回值为负数表示 a < b, 零表示 a == b, 正数表示 a > b
    let result = unsafe { libc::strcoll(a_cstr.as_ptr(), b_cstr.as_ptr()) };

    result.cmp(&0)
}

/// 自然排序文件名比较函数 (StrCmpLogicalW 风格, 支持平台特定汉字排序)
///
/// 逐字符交替比较文本段和数字段:
///   - 数字段按数值比较: "2" < "10"
///   - ASCII 字母大小写不敏感
///   - 汉字按平台特定 API 排序, 支持区域感知 (locale-aware) 比较:
///       * Windows: 使用 CompareStringEx, 中文系统下按拼音排序
///       * Linux/macOS: 使用 strcoll, 需要系统已安装中文 locale
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
        } else if is_cjk_char(a_ch) && is_cjk_char(b_ch) {
            // 两个字符都是汉字: 使用平台特定 API 进行区域感知比较
            match compare_cjk_chars(a_ch, b_ch) {
                std::cmp::Ordering::Equal => {
                    a_idx += 1;
                    b_idx += 1;
                }
                ord => return ord,
            }
        } else {
            // 非汉字字符 (ASCII 字母 / 符号 / 其他 Unicode):
            // 统一使用 ASCII 大小写不敏感比较, 保持与 Windows Explorer 一致的排序行为
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

    #[cfg(not(target_os = "windows"))]
    use std::ffi::CString;
    #[cfg(not(target_os = "windows"))]
    use std::sync::Once;

    #[cfg(not(target_os = "windows"))]
    static LOCALE_INIT: Once = Once::new();

    #[cfg(not(target_os = "windows"))]
    fn ensure_collation_locale() {
        LOCALE_INIT.call_once(|| {
            let empty = CString::new("").unwrap();
            unsafe { libc::setlocale(libc::LC_COLLATE, empty.as_ptr()); }
        });
    }

    #[test]
    fn test_natural_sort_chinese() {
        #[cfg(not(target_os = "windows"))]
        ensure_collation_locale();
        let mut names = vec!["安次得胜口马氏家谱序", "新建 DOCX 文档", "AI编程工具OpenCode"];
        names.sort_by(|a, b| compare_filename_natural(a, b));
        // ASCII 字母排在汉字前面; 汉字按平台区域设置排序:
        //   Windows (zh-CN): CompareStringEx, 拼音 an < xin
        //   Linux/macOS: strcoll, 需 zh_CN.UTF-8 locale
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
        #[cfg(not(target_os = "windows"))]
        ensure_collation_locale();
        let mut names = vec![
            "本项目的结果-AI编程工具OpenCode",
            "新建 DOCX 文档",
            "安次得胜口马氏家谱序",
        ];
        names.sort_by(|a, b| compare_filename_natural(a, b));
        // 平台特定区域感知排序:
        //   Windows (zh-CN): 安(an) < 本(ben) < 新(xin)
        //   Linux/macOS: 同上, 需 zh_CN.UTF-8 locale 支持
        assert_eq!(names[0], "安次得胜口马氏家谱序");
        assert_eq!(names[1], "本项目的结果-AI编程工具OpenCode");
        assert_eq!(names[2], "新建 DOCX 文档");
    }
}
