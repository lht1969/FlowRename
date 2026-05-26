// P1 Methods - Extended renaming methods (List, Move, Trim, Renumber, Timestamp)
// These methods provide advanced renaming capabilities beyond the core P0 set

use anyhow::{bail, Result};

use crate::method_engine::traits::{Method, MethodContext};
use crate::models::{
    ApplyToOption, MethodConfig, MethodType,
    ListConfig, ListOverflow, MoveConfig, TrimConfig,
    RenumberConfig, RenumberPosition, TimestampConfig, TimestampSource,
};

/// Split a full filename into name part and extension part
fn split_name_ext(input: &str, original_ext: &str) -> (String, String) {
    let ext_stripped = original_ext.trim_start_matches('.');
    if ext_stripped.is_empty() {
        return (input.to_string(), String::new());
    }
    if let Some(dot_pos) = input.rfind(&format!(".{}", ext_stripped)) {
        let (name, ext) = input.split_at(dot_pos);
        (name.to_string(), ext.to_string())
    } else {
        (input.to_string(), String::new())
    }
}

/// Move a range of characters within a string to a new position
fn move_chars(input: &str, from_start: usize, count: usize, to_position: usize) -> String {
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();

    if len == 0 {
        return input.to_string();
    }

    let from_start_clamped = from_start.min(len);
    let from_end = (from_start_clamped + count).min(len);
    let actual_count = from_end - from_start_clamped;

    if actual_count == 0 {
        return input.to_string();
    }

    let moved_text: String = chars[from_start_clamped..from_end].iter().collect();

    let mut remaining: Vec<char> = chars[..from_start_clamped].iter()
        .chain(chars[from_end..].iter())
        .copied()
        .collect();

    let insert_pos = to_position.min(remaining.len());
    remaining.splice(insert_pos..insert_pos, moved_text.chars());

    remaining.into_iter().collect()
}

// ==================== LIST METHOD ====================

/// List method - renames files using names from a provided list
/// Each file gets the corresponding name from the list (matched by index)
#[derive(Debug)]
pub struct ListMethod {
    config: ListConfig,
}

impl ListMethod {
    pub fn new(config: ListConfig) -> Self {
        Self { config }
    }
}

impl Method for ListMethod {
    fn name(&self) -> &str { "List" }

    fn method_type(&self) -> MethodType { MethodType::List }

    fn apply(&self, input: &str, context: &MethodContext) -> Result<String> {
        let (name_part, ext_part) = split_name_ext(input, &context.original_ext);
        let apply_to = &self.config.apply_to;
        let index = context.file_index;

        let new_name = if index < self.config.names.len() {
            self.config.names[index].clone()
        } else {
            match &self.config.overflow_behavior {
                ListOverflow::KeepOriginal => name_part.clone(),
                ListOverflow::Skip => name_part.clone(),
                ListOverflow::Cycle => {
                    let cycled_index = index % self.config.names.len();
                    self.config.names[cycled_index].clone()
                }
            }
        };

        match apply_to {
            ApplyToOption::Name => Ok(format!("{}{}", new_name, ext_part)),
            ApplyToOption::Extension => Ok(format!("{}{}", name_part, new_name)),
            ApplyToOption::Both => Ok(new_name),
        }
    }

    fn validate(&self) -> Result<()> {
        if self.config.names.is_empty() {
            bail!("Name list cannot be empty");
        }
        Ok(())
    }

    fn to_config(&self) -> MethodConfig {
        MethodConfig::List(self.config.clone())
    }

    fn apply_to(&self) -> ApplyToOption {
        self.config.apply_to.clone()
    }
}

// ==================== MOVE METHOD ====================

/// Move method - moves a range of characters within the filename to a new position
/// Useful for rearranging parts of filenames (e.g., moving date prefixes)
#[derive(Debug)]
pub struct MoveMethod {
    config: MoveConfig,
}

impl MoveMethod {
    pub fn new(config: MoveConfig) -> Self {
        Self { config }
    }
}

impl Method for MoveMethod {
    fn name(&self) -> &str { "Move" }

    fn method_type(&self) -> MethodType { MethodType::Move }

    fn apply(&self, input: &str, context: &MethodContext) -> Result<String> {
        let (name_part, ext_part) = split_name_ext(input, &context.original_ext);
        let apply_to = &self.config.apply_to;

        let target = match apply_to {
            ApplyToOption::Name => name_part.as_str(),
            ApplyToOption::Extension => ext_part.as_str(),
            ApplyToOption::Both => input,
        };

        let result = move_chars(target, self.config.from_start, self.config.count, self.config.to_position);

        match apply_to {
            ApplyToOption::Name => Ok(format!("{}{}", result, ext_part)),
            ApplyToOption::Extension => Ok(format!("{}{}", name_part, result)),
            ApplyToOption::Both => Ok(result),
        }
    }

    fn validate(&self) -> Result<()> {
        if self.config.count == 0 {
            bail!("Move count must be at least 1");
        }
        Ok(())
    }

    fn to_config(&self) -> MethodConfig {
        MethodConfig::Move(self.config.clone())
    }

    fn apply_to(&self) -> ApplyToOption {
        self.config.apply_to.clone()
    }
}

// ==================== TRIM METHOD ====================

/// Trim method - removes characters from the start and/or end of filenames
/// Can trim specific characters or whitespace
#[derive(Debug)]
pub struct TrimMethod {
    config: TrimConfig,
}

impl TrimMethod {
    pub fn new(config: TrimConfig) -> Self {
        Self { config }
    }
}

impl Method for TrimMethod {
    fn name(&self) -> &str { "Trim" }

    fn method_type(&self) -> MethodType { MethodType::Trim }

    fn apply(&self, input: &str, context: &MethodContext) -> Result<String> {
        let (name_part, ext_part) = split_name_ext(input, &context.original_ext);
        let apply_to = &self.config.apply_to;

        let target = match apply_to {
            ApplyToOption::Name => name_part.as_str(),
            ApplyToOption::Extension => ext_part.as_str(),
            ApplyToOption::Both => input,
        };

        let result = if self.config.trim_whitespace {
            target.trim().to_string()
        } else {
            let mut result = target.to_string();

            if !self.config.trim_start.is_empty() {
                let trim_chars: Vec<char> = self.config.trim_start.chars().collect();
                result = result.trim_start_matches(|c| trim_chars.contains(&c)).to_string();
            }

            if !self.config.trim_end.is_empty() {
                let trim_chars: Vec<char> = self.config.trim_end.chars().collect();
                result = result.trim_end_matches(|c| trim_chars.contains(&c)).to_string();
            }

            result
        };

        match apply_to {
            ApplyToOption::Name => Ok(format!("{}{}", result, ext_part)),
            ApplyToOption::Extension => Ok(format!("{}{}", name_part, result)),
            ApplyToOption::Both => Ok(result),
        }
    }

    fn validate(&self) -> Result<()> {
        if !self.config.trim_whitespace
            && self.config.trim_start.is_empty()
            && self.config.trim_end.is_empty()
        {
            bail!("At least one trim option must be specified");
        }
        Ok(())
    }

    fn to_config(&self) -> MethodConfig {
        MethodConfig::Trim(self.config.clone())
    }

    fn apply_to(&self) -> ApplyToOption {
        self.config.apply_to.clone()
    }
}

// ==================== RENUMBER METHOD ====================

/// Renumber method - inserts sequential numbers into filenames
/// Supports configurable start, step, padding, and position
#[derive(Debug)]
pub struct RenumberMethod {
    config: RenumberConfig,
}

impl RenumberMethod {
    pub fn new(config: RenumberConfig) -> Self {
        Self { config }
    }
}

impl Method for RenumberMethod {
    fn name(&self) -> &str { "Renumber" }

    fn method_type(&self) -> MethodType { MethodType::Renumber }

    fn apply(&self, input: &str, context: &MethodContext) -> Result<String> {
        let (name_part, ext_part) = split_name_ext(input, &context.original_ext);
        let apply_to = &self.config.apply_to;

        let number = self.config.start + (context.file_index * self.config.step);
        let formatted_number = format!("{:0>width$}", number, width = self.config.padding);

        let apply_renumber = |target: &str, _is_name: bool| -> String {
            match &self.config.position {
                RenumberPosition::Prefix => {
                    if self.config.separator.is_empty() {
                        format!("{}{}", formatted_number, target)
                    } else {
                        format!("{}{}{}", formatted_number, self.config.separator, target)
                    }
                }
                RenumberPosition::Suffix => {
                    if self.config.separator.is_empty() {
                        format!("{}{}", target, formatted_number)
                    } else {
                        format!("{}{}{}", target, self.config.separator, formatted_number)
                    }
                }
                RenumberPosition::Replace => {
                    formatted_number.clone()
                }
            }
        };

        match apply_to {
            ApplyToOption::Name => {
                let new_name = apply_renumber(&name_part, true);
                Ok(format!("{}{}", new_name, ext_part))
            }
            ApplyToOption::Extension => {
                let new_ext = apply_renumber(&ext_part, false);
                Ok(format!("{}{}", name_part, new_ext))
            }
            ApplyToOption::Both => {
                let new_full = apply_renumber(input, true);
                Ok(new_full)
            }
        }
    }

    fn validate(&self) -> Result<()> {
        if self.config.padding == 0 {
            bail!("Padding must be at least 1 digit");
        }
        if self.config.step == 0 {
            bail!("Step must be at least 1");
        }
        Ok(())
    }

    fn to_config(&self) -> MethodConfig {
        MethodConfig::Renumber(self.config.clone())
    }

    fn apply_to(&self) -> ApplyToOption {
        self.config.apply_to.clone()
    }
}

// ==================== TIMESTAMP METHOD ====================

/// Timestamp method - sets filename based on file's timestamp
/// Supports created, modified, and accessed timestamps with custom formatting
#[derive(Debug)]
pub struct TimestampMethod {
    config: TimestampConfig,
}

impl TimestampMethod {
    pub fn new(config: TimestampConfig) -> Self {
        Self { config }
    }
}

impl Method for TimestampMethod {
    fn name(&self) -> &str { "Timestamp" }

    fn method_type(&self) -> MethodType { MethodType::Timestamp }

    fn apply(&self, input: &str, context: &MethodContext) -> Result<String> {
        let (name_part, ext_part) = split_name_ext(input, &context.original_ext);
        let apply_to = &self.config.apply_to;

        let timestamp_str = match self.config.source {
            TimestampSource::Created => context.created_time.clone(),
            TimestampSource::Modified => context.modified_time.clone(),
            TimestampSource::Accessed => context.accessed_time.clone(),
        };

        let formatted = if let Some(ts) = timestamp_str {
            match chrono::NaiveDateTime::parse_from_str(&ts, "%Y-%m-%dT%H:%M:%S") {
                Ok(dt) => convert_chrono_format(&self.config.format, &dt),
                Err(_) => {
                    match chrono::NaiveDate::parse_from_str(&ts, "%Y-%m-%d") {
                        Ok(d) => convert_chrono_format_date(&self.config.format, &d),
                        Err(_) => ts,
                    }
                }
            }
        } else {
            let now = chrono::Local::now();
            convert_chrono_format(&self.config.format, &now.naive_local())
        };

        match apply_to {
            ApplyToOption::Name => Ok(format!("{}{}", formatted, ext_part)),
            ApplyToOption::Extension => Ok(format!("{}{}", name_part, formatted)),
            ApplyToOption::Both => Ok(formatted),
        }
    }

    fn validate(&self) -> Result<()> {
        if self.config.format.is_empty() {
            bail!("Format string cannot be empty");
        }
        Ok(())
    }

    fn to_config(&self) -> MethodConfig {
        MethodConfig::Timestamp(self.config.clone())
    }

    fn apply_to(&self) -> ApplyToOption {
        self.config.apply_to.clone()
    }
}

/// Convert our custom format string (YYYY-MM-DD style) to chrono format
/// We use a user-friendly format that gets translated to chrono's %Y-%m-%d style
pub fn convert_chrono_format_pub(format: &str, dt: &chrono::NaiveDateTime) -> String {
    convert_chrono_format(format, dt)
}

fn convert_chrono_format(format: &str, dt: &chrono::NaiveDateTime) -> String {
    let mut result = format.to_string();

    // Replace user-friendly tokens with chrono format codes
    result = result.replace("YYYY", &dt.format("%Y").to_string());
    result = result.replace("YY", &dt.format("%y").to_string());
    result = result.replace("MM", &dt.format("%m").to_string());
    result = result.replace("DD", &dt.format("%d").to_string());
    result = result.replace("HH", &dt.format("%H").to_string());
    result = result.replace("mm", &dt.format("%M").to_string());
    result = result.replace("ss", &dt.format("%S").to_string());

    result
}

/// Convert format string using date only (no time component)
fn convert_chrono_format_date(format: &str, date: &chrono::NaiveDate) -> String {
    let mut result = format.to_string();

    result = result.replace("YYYY", &date.format("%Y").to_string());
    result = result.replace("YY", &date.format("%y").to_string());
    result = result.replace("MM", &date.format("%m").to_string());
    result = result.replace("DD", &date.format("%d").to_string());
    result = result.replace("HH", "00");
    result = result.replace("mm", "00");
    result = result.replace("ss", "00");

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ApplyToOption;

    fn default_context(index: usize) -> MethodContext {
        MethodContext {
            file_index: index,
            total_files: 5,
            file_metadata: None,
            original_name: "test".to_string(),
            original_ext: ".txt".to_string(),
            created_time: None,
            modified_time: None,
            accessed_time: None,
        }
    }

    #[test]
    fn test_list_method_direct() {
        let config = ListConfig {
            names: vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()],
            overflow_behavior: ListOverflow::KeepOriginal,
            apply_to: ApplyToOption::Name,
        };
        let method = ListMethod::new(config);

        let ctx0 = default_context(0);
        let ctx1 = default_context(1);
        assert_eq!(method.apply("old", &ctx0).unwrap(), "alpha");
        assert_eq!(method.apply("old", &ctx1).unwrap(), "beta");
    }

    #[test]
    fn test_list_method_overflow_keep() {
        let config = ListConfig {
            names: vec!["alpha".to_string()],
            overflow_behavior: ListOverflow::KeepOriginal,
            apply_to: ApplyToOption::Name,
        };
        let method = ListMethod::new(config);

        let ctx = default_context(5);
        assert_eq!(method.apply("original", &ctx).unwrap(), "original");
    }

    #[test]
    fn test_list_method_overflow_cycle() {
        let config = ListConfig {
            names: vec!["alpha".to_string(), "beta".to_string()],
            overflow_behavior: ListOverflow::Cycle,
            apply_to: ApplyToOption::Name,
        };
        let method = ListMethod::new(config);

        let ctx = default_context(3); // 3 % 2 = 1 → "beta"
        assert_eq!(method.apply("old", &ctx).unwrap(), "beta");
    }

    #[test]
    fn test_move_method() {
        let config = MoveConfig {
            from_start: 0,
            count: 3,
            to_position: 5,
            apply_to: ApplyToOption::Name,
        };
        let method = MoveMethod::new(config);

        let ctx = default_context(0);
        // Move "ABC" from position 0 to position 5 (after removal)
        // "ABCDEF" → remove "ABC" → "DEF" → insert at pos 5? No, "DEF" is only 3 chars
        // Insert at min(5, 3) = 3 → "DEFABC"
        assert_eq!(method.apply("ABCDEF", &ctx).unwrap(), "DEFABC");
    }

    #[test]
    fn test_trim_method_whitespace() {
        let config = TrimConfig {
            trim_start: String::new(),
            trim_end: String::new(),
            trim_whitespace: true,
            apply_to: ApplyToOption::Name,
        };
        let method = TrimMethod::new(config);

        let ctx = default_context(0);
        assert_eq!(method.apply("  hello  ", &ctx).unwrap(), "hello");
    }

    #[test]
    fn test_trim_method_chars() {
        let config = TrimConfig {
            trim_start: "0123456789".to_string(),
            trim_end: "._- ".to_string(),
            trim_whitespace: false,
            apply_to: ApplyToOption::Name,
        };
        let method = TrimMethod::new(config);

        let ctx = default_context(0);
        assert_eq!(method.apply("001photo.jpg_", &ctx).unwrap(), "photo.jpg");
    }

    #[test]
    fn test_renumber_method_prefix() {
        let config = RenumberConfig {
            start: 1,
            step: 1,
            padding: 3,
            position: RenumberPosition::Prefix,
            separator: "_".to_string(),
            apply_to: ApplyToOption::Name,
        };
        let method = RenumberMethod::new(config);

        let ctx0 = default_context(0);
        let ctx4 = default_context(4);
        assert_eq!(method.apply("photo.jpg", &ctx0).unwrap(), "001_photo.jpg");
        assert_eq!(method.apply("photo.jpg", &ctx4).unwrap(), "005_photo.jpg");
    }

    #[test]
    fn test_renumber_method_replace() {
        let config = RenumberConfig {
            start: 10,
            step: 5,
            padding: 2,
            position: RenumberPosition::Replace,
            separator: String::new(),
            apply_to: ApplyToOption::Name,
        };
        let method = RenumberMethod::new(config);

        let ctx = default_context(0);
        assert_eq!(method.apply("photo.jpg", &ctx).unwrap(), "10.jpg");
    }

    #[test]
    fn test_timestamp_method() {
        let config = TimestampConfig {
            source: TimestampSource::Modified,
            format: "YYYY-MM-DD".to_string(),
            apply_to: ApplyToOption::Name,
        };
        let method = TimestampMethod::new(config);

        // Without metadata, it uses current time
        let ctx = default_context(0);
        let result = method.apply("photo.jpg", &ctx).unwrap();
        // Should end with .jpg and have a date-like format
        assert!(result.ends_with(".jpg"));
        assert!(result.len() > 8); // At least "YYYY-MM-DD.jpg"
    }
}
