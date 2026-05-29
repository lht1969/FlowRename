use anyhow::{bail, Result};
use regex::Regex;

use crate::method_engine::traits::{Method, MethodContext};
use crate::models::{
    ApplyToOption, CaseType, CaseLocation, MethodConfig, MethodType,
    OccurrenceOption, RemovePosition, ReplaceConfig, AddConfig, RemoveConfig,
    NewCaseConfig, NewNameConfig, AddPosition,
};

// ==================== HELPER FUNCTIONS ====================

/// Split a full filename into name part and extension part
/// Uses the original extension from context to correctly handle multi-dot extensions
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

/// Insert text at a specific character position (handles Unicode correctly)
fn insert_at_pos(input: &str, text: &str, pos: usize) -> String {
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let actual_pos = pos.min(len);
    if actual_pos == 0 {
        format!("{}{}", text, input)
    } else if actual_pos >= len {
        format!("{}{}", input, text)
    } else {
        let mut result = chars[..actual_pos].to_vec();
        result.extend(text.chars());
        result.extend_from_slice(&chars[actual_pos..]);
        result.into_iter().collect()
    }
}

/// Insert text at the end of the input (handles backwards position for Add method)
fn insert_at_end(input: &str, text: &str) -> String {
    format!("{}{}", input, text)
}

// ==================== REPLACE METHOD ====================

/// Replace method - finds and replaces text patterns in filenames
/// Supports plain text and regular expressions with various occurrence options
#[derive(Debug)]
pub struct ReplaceMethod {
    config: ReplaceConfig,
}

impl ReplaceMethod {
    /// Create a new Replace method with the given configuration
    pub fn new(config: ReplaceConfig) -> Self {
        Self { config }
    }

    /// Apply replacement logic to a target string (name part, extension part, or full name)
    fn apply_to_target(&self, target: &str) -> Result<String> {
        if self.config.use_regex {
            let flags = if self.config.case_sensitive { "" } else { "(?i)" };
            let pattern = format!("{}{}", flags, self.config.find);
            let re = Regex::new(&pattern).map_err(|e| {
                anyhow::anyhow!("Invalid regex pattern '{}': {}", self.config.find, e)
            })?;

            match &self.config.occurrence {
                OccurrenceOption::All => Ok(re.replace_all(target, &self.config.replace_with).to_string()),
                OccurrenceOption::First => Ok(re.replace(target, &self.config.replace_with).to_string()),
                OccurrenceOption::Last => {
                    let matches: Vec<_> = re.find_iter(target).collect();
                    if let Some(last_match) = matches.last() {
                        let before = &target[..last_match.start()];
                        let after = &target[last_match.end()..];
                        Ok(format!("{}{}{}", before, self.config.replace_with, after))
                    } else {
                        Ok(target.to_string())
                    }
                }
                OccurrenceOption::Custom(n) => {
                    let mut count = 0;
                    for m in re.find_iter(target) {
                        count += 1;
                        if count == *n {
                            let before = &target[..m.start()];
                            let after = &target[m.end()..];
                            return Ok(format!("{}{}{}", before, self.config.replace_with, after));
                        }
                    }
                    Ok(target.to_string())
                }
            }
        } else {
            let find = &self.config.find;
            let replace_with = &self.config.replace_with;

            match &self.config.occurrence {
                OccurrenceOption::All => {
                    if self.config.case_sensitive {
                        Ok(target.replace(find, replace_with))
                    } else {
                        let mut result = target.to_string();
                        let mut offset = 0;
                        while let Some(pos) = result[offset..].to_lowercase().find(&find.to_lowercase()) {
                            let actual_pos = offset + pos;
                            result.replace_range(actual_pos..actual_pos + find.len(), replace_with);
                            offset = actual_pos + replace_with.len();
                        }
                        Ok(result)
                    }
                }
                OccurrenceOption::First => {
                    let pos = if self.config.case_sensitive {
                        target.find(find)
                    } else {
                        target.to_lowercase().find(&find.to_lowercase())
                    };
                    if let Some(pos) = pos {
                        let mut result = target.to_string();
                        result.replace_range(pos..pos + find.len(), replace_with);
                        Ok(result)
                    } else {
                        Ok(target.to_string())
                    }
                }
                OccurrenceOption::Last => {
                    let pos = if self.config.case_sensitive {
                        target.rfind(find)
                    } else {
                        target.to_lowercase().rfind(&find.to_lowercase())
                    };
                    if let Some(pos) = pos {
                        let mut result = target.to_string();
                        result.replace_range(pos..pos + find.len(), replace_with);
                        Ok(result)
                    } else {
                        Ok(target.to_string())
                    }
                }
                OccurrenceOption::Custom(n) => {
                    let mut count = 0;
                    let mut search_start = 0;
                    loop {
                        let pos = if self.config.case_sensitive {
                            target[search_start..].find(find)
                        } else {
                            target[search_start..].to_lowercase().find(&find.to_lowercase())
                        };
                        match pos {
                            Some(relative_pos) => {
                                count += 1;
                                let actual_pos = search_start + relative_pos;
                                if count == *n {
                                    let mut result = target.to_string();
                                    result.replace_range(actual_pos..actual_pos + find.len(), replace_with);
                                    return Ok(result);
                                }
                                search_start = actual_pos + find.len();
                            }
                            None => break,
                        }
                    }
                    Ok(target.to_string())
                }
            }
        }
    }
}

impl Method for ReplaceMethod {
    fn name(&self) -> &str {
        "Replace"
    }

    fn method_type(&self) -> MethodType {
        MethodType::Replace
    }

    fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    fn apply(&self, input: &str, context: &MethodContext) -> Result<String> {
        let (name_part, ext_part) = split_name_ext(input, &context.original_ext);
        let apply_to = &self.config.apply_to;

        let target = match apply_to {
            ApplyToOption::Name => name_part.as_str(),
            ApplyToOption::Extension => ext_part.as_str(),
            ApplyToOption::Both => input,
        };

        let result = self.apply_to_target(target)?;

        match apply_to {
            ApplyToOption::Name => Ok(format!("{}{}", result, ext_part)),
            ApplyToOption::Extension => Ok(format!("{}{}", name_part, result)),
            ApplyToOption::Both => Ok(result),
        }
    }

    fn validate(&self) -> Result<()> {
        if self.config.find.is_empty() {
            bail!("Search pattern cannot be empty");
        }

        // Validate regex pattern if using regex mode
        if self.config.use_regex {
            Regex::new(&self.config.find).map_err(|e| {
                anyhow::anyhow!("Invalid regex pattern: {}", e)
            })?;
        }

        Ok(())
    }

    fn to_config(&self) -> MethodConfig {
        MethodConfig::Replace(self.config.clone())
    }

    fn apply_to(&self) -> ApplyToOption {
        self.config.apply_to.clone()
    }
}

// ==================== NEWNAME METHOD ADAPTER ====================

/// NewName method adapter - generates new filenames using templates with tags
/// Replaces tags like <Name>, <Ext>, <Index>, <Inc:N>, <Date:...> with actual values
#[derive(Debug)]
pub struct NewNameMethodAdapter {
    config: NewNameConfig,
}

impl NewNameMethodAdapter {
    pub fn new(config: NewNameConfig) -> Self {
        Self { config }
    }
}

impl Method for NewNameMethodAdapter {
    fn name(&self) -> &str { "NewName" }

    fn method_type(&self) -> MethodType { MethodType::NewName }

    fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    fn apply(&self, _input: &str, context: &MethodContext) -> Result<String> {
        use crate::tag_system::TagEvaluator;
        let evaluator = TagEvaluator::new();
        evaluator.evaluate_template(&self.config.template, context)
    }

    fn validate(&self) -> Result<()> {
        if self.config.template.is_empty() {
            bail!("Template cannot be empty");
        }
        Ok(())
    }

    fn to_config(&self) -> MethodConfig {
        MethodConfig::NewName(self.config.clone())
    }

    fn apply_to(&self) -> ApplyToOption {
        self.config.apply_to.clone()
    }
}

// ==================== ADD METHOD ====================

/// Add method - inserts text at specified positions in filenames
/// Supports adding at start, end, before/after extension, or custom position
#[derive(Debug)]
pub struct AddMethod {
    config: AddConfig,
}

impl AddMethod {
    /// Create a new Add method with the given configuration
    pub fn new(config: AddConfig) -> Self {
        Self { config }
    }
}

impl Method for AddMethod {
    fn name(&self) -> &str {
        "Add"
    }

    fn method_type(&self) -> MethodType {
        MethodType::Add
    }

    fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    fn apply(&self, input: &str, context: &MethodContext) -> Result<String> {
        let text = &self.config.text;
        let apply_to = &self.config.apply_to;

        // Split input into stem and extension
        let (name_part, ext_part) = split_name_ext(input, &context.original_ext);

        // Extension part starts with '.', e.g. ".txt"
        // When adding to extension, we want to add to the actual extension text (after the dot)
        let ext_chars: Vec<char> = ext_part.chars().collect();
        let has_dot = !ext_chars.is_empty() && ext_chars[0] == '.';
        let actual_ext: String = if has_dot {
            ext_chars.iter().skip(1).collect()
        } else {
            ext_part.clone()
        };

        // Reassemble extension with leading dot preserved
        let reassemble_ext = |new_ext_content: &str| -> String {
            if has_dot {
                format!(".{}", new_ext_content)
            } else {
                new_ext_content.to_string()
            }
        };

        match &self.config.position {
            AddPosition::Start => {
                match apply_to {
                    ApplyToOption::Name => {
                        let new_name = if self.config.backwards {
                            insert_at_end(&name_part, text)
                        } else {
                            format!("{}{}", text, name_part)
                        };
                        Ok(format!("{}{}", new_name, ext_part))
                    }
                    ApplyToOption::Extension => {
                        let new_ext_content = if self.config.backwards {
                            insert_at_end(&actual_ext, text)
                        } else {
                            format!("{}{}", text, &actual_ext)
                        };
                        Ok(format!("{}{}", name_part, reassemble_ext(&new_ext_content)))
                    }
                    ApplyToOption::Both => {
                        let new_full = if self.config.backwards {
                            insert_at_end(input, text)
                        } else {
                            format!("{}{}", text, input)
                        };
                        Ok(new_full)
                    }
                }
            }
            AddPosition::End => {
                match apply_to {
                    ApplyToOption::Name => {
                        let new_name = if self.config.backwards {
                            insert_at_end(&name_part, text)
                        } else {
                            format!("{}{}", name_part, text)
                        };
                        Ok(format!("{}{}", new_name, ext_part))
                    }
                    ApplyToOption::Extension => {
                        let new_ext_content = if self.config.backwards {
                            insert_at_end(&actual_ext, text)
                        } else {
                            format!("{}{}", &actual_ext, text)
                        };
                        Ok(format!("{}{}", name_part, reassemble_ext(&new_ext_content)))
                    }
                    ApplyToOption::Both => {
                        let new_full = if self.config.backwards {
                            insert_at_end(input, text)
                        } else {
                            format!("{}{}", input, text)
                        };
                        Ok(new_full)
                    }
                }
            }
            AddPosition::BeforeExt => {
                match apply_to {
                    ApplyToOption::Name => {
                        let new_name = if self.config.backwards {
                            insert_at_end(&name_part, text)
                        } else {
                            format!("{}{}", name_part, text)
                        };
                        Ok(format!("{}{}", new_name, ext_part))
                    }
                    ApplyToOption::Extension => {
                        // BeforeExt means before extension content, so same logic as Start
                        let new_ext_content = if self.config.backwards {
                            insert_at_end(&actual_ext, text)
                        } else {
                            format!("{}{}", text, &actual_ext)
                        };
                        Ok(format!("{}{}", name_part, reassemble_ext(&new_ext_content)))
                    }
                    ApplyToOption::Both => {
                         let new_full = if self.config.backwards {
                             insert_at_end(input, text)
                         } else {
                             let dot_pos = if !ext_part.is_empty() { input.rfind('.') } else { None };
                             if let Some(pos) = dot_pos {
                                 let (n, e) = input.split_at(pos);
                                 format!("{}{}{}", n, text, e)
                             } else {
                                 format!("{}{}", input, text)
                             }
                         };
                        Ok(new_full)
                    }
                }
            }
            AddPosition::AfterExt => {
                match apply_to {
                    ApplyToOption::Name => {
                        Ok(format!("{}{}", name_part, text))
                    }
                    ApplyToOption::Extension => {
                        // AfterExt means after extension content, so same logic as End
                        let new_ext_content = if self.config.backwards {
                            insert_at_end(&actual_ext, text)
                        } else {
                            format!("{}{}", &actual_ext, text)
                        };
                        Ok(format!("{}{}", name_part, reassemble_ext(&new_ext_content)))
                    }
                    ApplyToOption::Both => {
                        let new_full = if self.config.backwards {
                            insert_at_end(input, text)
                        } else {
                            format!("{}{}", input, text)
                        };
                        Ok(new_full)
                    }
                }
            }
            AddPosition::Custom(pos) => {
                match apply_to {
                    ApplyToOption::Name => {
                        let new_name = if self.config.backwards {
                            let actual = name_part.chars().count().saturating_sub(*pos);
                            insert_at_pos(&name_part, text, actual)
                        } else {
                            insert_at_pos(&name_part, text, *pos)
                        };
                        Ok(format!("{}{}", new_name, ext_part))
                    }
                    ApplyToOption::Extension => {
                        // Custom position on actual extension content
                        let new_ext_content = if self.config.backwards {
                            let actual = actual_ext.chars().count().saturating_sub(*pos);
                            insert_at_pos(&actual_ext, text, actual)
                        } else {
                            insert_at_pos(&actual_ext, text, *pos)
                        };
                        Ok(format!("{}{}", name_part, reassemble_ext(&new_ext_content)))
                    }
                    ApplyToOption::Both => {
                        let new_full = if self.config.backwards {
                            let actual = input.chars().count().saturating_sub(*pos);
                            insert_at_pos(input, text, actual)
                        } else {
                            insert_at_pos(input, text, *pos)
                        };
                        Ok(new_full)
                    }
                }
            }
        }
    }

    fn validate(&self) -> Result<()> {
        if self.config.text.is_empty() {
            bail!("Text to add cannot be empty");
        }
        Ok(())
    }

    fn to_config(&self) -> MethodConfig {
        MethodConfig::Add(self.config.clone())
    }

    fn apply_to(&self) -> ApplyToOption {
        self.config.apply_to.clone()
    }
}

// ==================== REMOVE METHOD ====================

/// Remove method - removes characters from filenames based on various criteria
/// Supports removal by position range, pattern matching, or character type
#[derive(Debug)]
pub struct RemoveMethod {
    config: RemoveConfig,
}

impl RemoveMethod {
    /// Create a new Remove method with the given configuration
    pub fn new(config: RemoveConfig) -> Self {
        Self { config }
    }
}

impl Method for RemoveMethod {
    fn name(&self) -> &str {
        "Remove"
    }

    fn method_type(&self) -> MethodType {
        MethodType::Remove
    }

    fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    fn apply(&self, input: &str, context: &MethodContext) -> Result<String> {
        let (name_part, ext_part) = split_name_ext(input, &context.original_ext);
        let apply_to = &self.config.apply_to;

        let result = match apply_to {
            ApplyToOption::Name => {
                let target = name_part.as_str();
                match &self.config.position {
                    RemovePosition::Start => {
                        let count = self.config.count.min(target.chars().count());
                        let chars: Vec<char> = target.chars().collect();
                        let new_name: String = chars.iter().skip(count).collect();
                        format!("{}{}", new_name, ext_part)
                    }
                    RemovePosition::End => {
                        let count = self.config.count.min(target.chars().count());
                        let chars: Vec<char> = target.chars().collect();
                        let keep = chars.len().saturating_sub(count);
                        let new_name: String = chars.iter().take(keep).collect();
                        format!("{}{}", new_name, ext_part)
                    }
                }
            }
            ApplyToOption::Extension => {
                // Extension part starts with '.', e.g. ".txt"
                // When removing from extension, skip the leading '.' and operate on the actual extension text
                let ext_chars: Vec<char> = ext_part.chars().collect();
                let has_dot = !ext_chars.is_empty() && ext_chars[0] == '.';
                let actual_ext: String = if has_dot {
                    ext_chars.iter().skip(1).collect()
                } else {
                    ext_part.clone()
                };

                let new_ext_content: String = match &self.config.position {
                    RemovePosition::Start => {
                        let count = self.config.count.min(actual_ext.chars().count());
                        let chars: Vec<char> = actual_ext.chars().collect();
                        chars.iter().skip(count).collect()
                    }
                    RemovePosition::End => {
                        let count = self.config.count.min(actual_ext.chars().count());
                        let chars: Vec<char> = actual_ext.chars().collect();
                        let keep = chars.len().saturating_sub(count);
                        chars.iter().take(keep).collect()
                    }
                };

                // Reassemble: keep the leading '.' + remaining extension content
                if has_dot {
                    format!("{}.{}", name_part, new_ext_content)
                } else {
                    format!("{}{}", name_part, new_ext_content)
                }
            }
            ApplyToOption::Both => {
                let target = input;
                match &self.config.position {
                    RemovePosition::Start => {
                        let count = self.config.count.min(target.chars().count());
                        let chars: Vec<char> = target.chars().collect();
                        chars.iter().skip(count).collect()
                    }
                    RemovePosition::End => {
                        let count = self.config.count.min(target.chars().count());
                        let chars: Vec<char> = target.chars().collect();
                        let keep = chars.len().saturating_sub(count);
                        chars.iter().take(keep).collect()
                    }
                }
            }
        };

        Ok(result)
    }

    fn validate(&self) -> Result<()> {
        // Basic validation - can be extended based on specific remove type requirements
        Ok(())
    }

    fn to_config(&self) -> MethodConfig {
        MethodConfig::Remove(self.config.clone())
    }

    fn apply_to(&self) -> ApplyToOption {
        self.config.apply_to.clone()
    }
}

// ==================== NEWCASE METHOD ====================

/// NewCase method - changes capitalization of filenames
/// Supports uppercase, lowercase, title case, sentence case, and toggle case
#[derive(Debug)]
pub struct NewCaseMethod {
    config: NewCaseConfig,
}

impl NewCaseMethod {
    /// Create a new NewCase method with the given configuration
    pub fn new(config: NewCaseConfig) -> Self {
        Self { config }
    }
}

impl Method for NewCaseMethod {
    fn name(&self) -> &str {
        "NewCase"
    }

    fn method_type(&self) -> MethodType {
        MethodType::NewCase
    }

    fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    fn apply(&self, input: &str, context: &MethodContext) -> Result<String> {
        let (name_part, ext_part) = split_name_ext(input, &context.original_ext);
        let apply_to = &self.config.apply_to;

        let apply_case = |s: &str| -> String {
            let transformed = match &self.config.new_case {
                CaseType::Lower => s.to_lowercase(),
                CaseType::Upper => s.to_uppercase(),
                CaseType::Title => {
                    // 按字母连续段拆分，支持空格、-、_ 等多种分隔符
                    let mut result = String::new();
                    let mut in_word = false;

                    for c in s.chars() {
                        if c.is_alphabetic() {
                            if !in_word {
                                // 单词首字母
                                in_word = true;
                                result.push_str(&c.to_uppercase().to_string());
                            } else {
                                // 单词后续字母保持原样
                                result.push(c);
                            }
                        } else {
                            // 非字母字符作为分隔符
                            in_word = false;
                            result.push(c);
                        }
                    }

                    result
                }
                CaseType::Sentence => {
                    let mut result = String::new();
                    let mut new_sentence = true;
                    for c in s.chars() {
                        if new_sentence && c.is_alphabetic() {
                            result.push(c.to_uppercase().next().unwrap_or(c));
                            new_sentence = false;
                        } else {
                            result.push(c);
                            if c == '.' || c == '!' || c == '?' {
                                new_sentence = true;
                            }
                        }
                    }
                    result
                }
                CaseType::Inverted => {
                    s.chars()
                        .map(|c| {
                            if c.is_uppercase() {
                                c.to_lowercase().next().unwrap_or(c)
                            } else if c.is_lowercase() {
                                c.to_uppercase().next().unwrap_or(c)
                            } else {
                                c
                            }
                        })
                        .collect()
                }
            };

            match &self.config.location {
                CaseLocation::All => transformed,
                CaseLocation::FirstLetter => {
                    let mut chars = s.chars();
                    match chars.next() {
                        Some(first) => {
                            let first_changed = match &self.config.new_case {
                                CaseType::Upper | CaseType::Title | CaseType::Sentence => {
                                    first.to_uppercase().to_string()
                                }
                                CaseType::Lower => first.to_lowercase().to_string(),
                                CaseType::Inverted => {
                                    if first.is_uppercase() {
                                        first.to_lowercase().to_string()
                                    } else if first.is_lowercase() {
                                        first.to_uppercase().to_string()
                                    } else {
                                        first.to_string()
                                    }
                                }
                            };
                            let rest: String = chars.collect();
                            format!("{}{}", first_changed, rest)
                        }
                        None => String::new(),
                    }
                }
                CaseLocation::EveryWordFirstLetter => {
                    let mut result = String::new();
                    let mut in_word = false;

                    for c in s.chars() {
                        if c.is_alphabetic() {
                            if !in_word {
                                in_word = true;
                                let changed = match &self.config.new_case {
                                    CaseType::Upper | CaseType::Title | CaseType::Sentence => {
                                        c.to_uppercase().to_string()
                                    }
                                    CaseType::Lower => c.to_lowercase().to_string(),
                                    CaseType::Inverted => {
                                        if c.is_uppercase() {
                                            c.to_lowercase().to_string()
                                        } else if c.is_lowercase() {
                                            c.to_uppercase().to_string()
                                        } else {
                                            c.to_string()
                                        }
                                    }
                                };
                                result.push_str(&changed);
                            } else {
                                result.push(c);
                            }
                        } else {
                            in_word = false;
                            result.push(c);
                        }
                    }

                    result
                }
                CaseLocation::ByPattern => transformed,
                CaseLocation::ByPosition => transformed,
            }
        };

        match apply_to {
            ApplyToOption::Name => {
                let transformed = apply_case(&name_part);
                Ok(format!("{}{}", transformed, ext_part))
            }
            ApplyToOption::Extension => {
                let transformed = apply_case(&ext_part);
                Ok(format!("{}{}", name_part, transformed))
            }
            ApplyToOption::Both => {
                let transformed = apply_case(input);
                Ok(transformed)
            }
        }
    }

    fn validate(&self) -> Result<()> {
        // All case types are valid by default
        Ok(())
    }

    fn to_config(&self) -> MethodConfig {
        MethodConfig::NewCase(self.config.clone())
    }

    fn apply_to(&self) -> ApplyToOption {
        self.config.apply_to.clone()
    }
}
