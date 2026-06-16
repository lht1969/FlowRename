/// Tag token representing a parsed tag from a template string
#[derive(Debug, Clone)]
pub struct TagToken {
    /// Name of the tag (e.g., "Date", "Inc", "Name")
    pub tag_name: String,
    
    /// Modifiers/parameters for the tag (e.g., ["YYYYMMDD"] for Date:YYYYMMDD)
    pub modifiers: Vec<String>,
    
    /// Original text representation of this tag
    pub original_text: String,
}

/// Parser for extracting tags from template strings
/// Supports syntax: <TagName> or <TagName:modifier1:modifier2>
pub struct TagParser;

impl TagParser {
    /// Create a new TagParser instance
    pub fn new() -> Self {
        Self
    }

    /// Parse a template string and extract all tags
    /// 
    /// # Arguments
    /// * `template` - The template string containing tags in angle brackets
    /// 
    /// # Returns
    /// A vector of `TagToken` objects representing all found tags.
    /// Malformed tags are silently ignored.
    /// 
    /// # Examples
    /// ```ignore
    /// let parser = TagParser::new();
    /// let tokens = parser.parse("<Date:YYYY>_<Inc:3>_photo");
    /// // Returns 2 tokens: Date with modifier ["YYYY"], Inc with modifier ["3"]
    /// ```
    pub fn parse(&self, template: &str) -> Vec<TagToken> {
        let mut tokens = Vec::new();
        let mut chars = template.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '<' {
                // Start of potential tag
                if let Some(tag_content) = self.extract_tag_content(&mut chars) {
                    if let Some(token) = self.parse_tag_content(&tag_content) {
                        tokens.push(token);
                    }
                }
            }
            // Regular characters are ignored (we only care about tags)
        }

        tokens
    }

    /// Extract content between angle brackets
    fn extract_tag_content<I>(&self, chars: &mut std::iter::Peekable<I>) -> Option<String>
    where
        I: Iterator<Item = char>,
    {
        let mut content = String::new();
        let mut depth = 1;  // We've already consumed the opening '<'

        for c in chars.by_ref() {
            match c {
                '<' => {
                    depth += 1;
                    content.push(c);
                }
                '>' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(content);
                    }
                    content.push(c);
                }
                _ => content.push(c),
            }
        }

        None  // Unclosed tag - return None to indicate malformed input
    }

    /// Parse the content inside angle brackets into a TagToken
    fn parse_tag_content(&self, content: &str) -> Option<TagToken> {
        let trimmed = content.trim();
        
        if trimmed.is_empty() {
            return None;
        }

        // Split on colon to get tag name and modifiers
        let parts: Vec<&str> = trimmed.split(':').collect();
        
        if parts.is_empty() || parts[0].is_empty() {
            return None;
        }

        let tag_name = parts[0].trim().to_string();
        let modifiers: Vec<String> = parts[1..]
            .iter()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Some(TagToken {
            tag_name,
            modifiers,
            original_text: format!("<{}>", trimmed),
        })
    }
}

impl Default for TagParser {
    fn default() -> Self {
        Self::new()
    }
}
