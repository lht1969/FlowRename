use serde::{Deserialize, Serialize};

/// Apply scope for renaming operations
/// Determines which part(s) of the filename the method affects
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[derive(Default)]
pub enum ApplyToOption {
    /// Apply only to the name part (before the extension)
    Name,

    /// Apply only to the extension (including dot)
    Extension,

    /// Apply to both name and extension
    #[default]
    Both,
}


impl std::fmt::Display for ApplyToOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplyToOption::Name => write!(f, "Name"),
            ApplyToOption::Extension => write!(f, "Extension"),
            ApplyToOption::Both => write!(f, "Both"),
        }
    }
}

// ==================== OCCURRENCE OPTIONS ====================

/// Controls which occurrence of a pattern to replace (for Replace method)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OccurrenceOption {
    /// Replace all occurrences
    All,

    /// Replace only the first occurrence
    First,

    /// Replace only the last occurrence
    Last,

    /// Replace a specific occurrence by index (1-based)
    Custom(usize),
}

// ==================== ADD POSITION OPTIONS ====================

/// Position where text should be added (for Add method)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AddPosition {
    /// Add at the beginning of the string
    Start,

    /// Add at the end of the string
    End,

    /// Add just before the extension (e.g., "photo" + "_new" + ".jpg")
    BeforeExt,

    /// Add after the extension (rarely used)
    AfterExt,

    /// Add at a custom character index (0-based)
    Custom(usize),
}

// ==================== REMOVE TYPE OPTIONS ====================

/// Type of removal operation (for Remove method)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RemoveType {
    /// Remove characters by position/range
    #[serde(rename = "position")]
    Position,
    
    /// Remove text matching a pattern
    #[serde(rename = "pattern")]
    Pattern,
    
    /// Remove specific character types (digits, letters, symbols, etc.)
    #[serde(rename = "type")]
    Type,
}

// ==================== CASE TYPE OPTIONS ====================

/// Target case transformation (for NewCase method)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CaseType {
    /// Convert to all lowercase: "HELLO" → "hello"
    Lower,
    
    /// Convert to all uppercase: "hello" → "HELLO"
    Upper,
    
    /// Convert to title case: "hello world" → "Hello World"
    Title,
    
    /// Invert case: "Hello" → "hELLO"
    Inverted,
    
    /// Sentence case: "hello. world." → "Hello. World."
    Sentence,
}

/// Location/Scope for case transformation (for NewCase method)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CaseLocation {
    /// Apply to entire string
    All,
    
    /// Only change the first character
    FirstLetter,
    
    /// Change first letter of every word
    EveryWordFirstLetter,
    
    /// Apply to parts matching a pattern
    ByPattern,
    
    /// Apply to characters at specific positions
    ByPosition,
}

// ==================== METHOD CONFIGURATIONS ====================

fn default_true() -> bool { true }

/// Configuration for the Replace method
/// Replaces text patterns in filenames using plain text or regex
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceConfig {
    /// Whether this method is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Text or regex pattern to find
    pub find: String,
    
    /// Replacement text (supports regex groups like $1, $2)
    pub replace_with: String,
    
    /// Which occurrence(s) to replace
    pub occurrence: OccurrenceOption,
    
    /// Whether search is case-sensitive
    pub case_sensitive: bool,
    
    /// Whether to interpret `find` as a regular expression
    pub use_regex: bool,
    
    /// Which part of the filename to apply replacement to
    pub apply_to: ApplyToOption,
}

/// Configuration for the Add method
/// Inserts text at specified positions in filenames
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddConfig {
    /// Whether this method is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Text to insert
    pub text: String,
    
    /// Where to insert the text
    pub position: AddPosition,
    
    /// Custom index when position is CustomIndex (0-based)
    pub custom_index: Option<usize>,
    
    /// For Start/End positions, insert from right side instead of left
    pub backwards: bool,
    
    /// Which part of the filename to apply insertion to
    pub apply_to: ApplyToOption,
}

/// Position for Remove method (where to remove characters from)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RemovePosition {
    Start,
    End,
}

/// Configuration for the Remove method
/// Removes characters from filenames based on position
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveConfig {
    /// Whether this method is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Number of characters to remove
    pub count: usize,
    
    /// Where to remove characters from
    pub position: RemovePosition,
    
    /// Which part of the filename to apply removal to
    pub apply_to: ApplyToOption,
}

/// Configuration for the NewCase method
/// Changes the capitalization of filenames
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCaseConfig {
    /// Whether this method is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Target case transformation type
    pub new_case: CaseType,
    
    /// Scope/location for case changes
    pub location: CaseLocation,
    
    /// Which part of the filename to apply case changes to
    pub apply_to: ApplyToOption,
}

/// Configuration for the NewName method
/// Generates completely new filenames using templates with tags
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewNameConfig {
    /// Whether this method is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Template string containing tags like "<Date:YYYYMMDD>_<Inc:3>"
    pub template: String,
    
    /// Which part of the filename this template generates
    pub apply_to: ApplyToOption,
}

/// Configuration for the List method
/// Renames files using names from a text list (one name per line)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListConfig {
    /// Whether this method is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// List of new names (one per line, matched by file index)
    pub names: Vec<String>,
    
    /// What to do if list has fewer names than files
    #[serde(default = "default_list_overflow")]
    pub overflow_behavior: ListOverflow,
    
    /// Which part of the filename to apply to
    pub apply_to: ApplyToOption,
}

/// Behavior when the name list has fewer entries than files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ListOverflow {
    /// Keep original name for files without a list entry
    KeepOriginal,
    /// Skip files without a list entry
    Skip,
    /// Cycle through the list from the beginning
    Cycle,
}

fn default_list_overflow() -> ListOverflow { ListOverflow::KeepOriginal }

/// Configuration for the Move method
/// Moves a range of characters within the filename to a new position
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveConfig {
    /// Whether this method is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Start position of the text to move (0-based)
    pub from_start: usize,
    
    /// Number of characters to move
    pub count: usize,
    
    /// Target position to insert the moved text (0-based, before removal)
    pub to_position: usize,
    
    /// Which part of the filename to apply to
    pub apply_to: ApplyToOption,
}

/// Configuration for the Trim method
/// Removes characters from the start and/or end of filenames
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrimConfig {
    /// Whether this method is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Characters to trim from the start (empty = no trim)
    #[serde(default)]
    pub trim_start: String,
    
    /// Characters to trim from the end (empty = no trim)
    #[serde(default)]
    pub trim_end: String,
    
    /// Whether to trim whitespace instead of specific characters
    #[serde(default)]
    pub trim_whitespace: bool,
    
    /// Which part of the filename to apply to
    pub apply_to: ApplyToOption,
}

/// Configuration for the Renumber method
/// Inserts sequential numbers into filenames
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenumberConfig {
    /// Whether this method is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Starting number for the sequence
    #[serde(default = "default_renumber_start")]
    pub start: usize,
    
    /// Step/increment between numbers
    #[serde(default = "default_renumber_step")]
    pub step: usize,
    
    /// Number of digits for zero-padding (e.g., 3 → "001")
    #[serde(default = "default_renumber_padding")]
    pub padding: usize,
    
    /// Position to insert the number
    #[serde(default = "default_renumber_position")]
    pub position: RenumberPosition,
    
    /// Separator between number and filename
    #[serde(default)]
    pub separator: String,
    
    /// Which part of the filename to apply to
    pub apply_to: ApplyToOption,
}

/// Position where the number should be inserted
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RenumberPosition {
    /// Insert before the filename (prefix)
    Prefix,
    /// Insert after the filename (suffix, before extension)
    Suffix,
    /// Replace the entire filename with just the number
    Replace,
}

fn default_renumber_start() -> usize { 1 }
fn default_renumber_step() -> usize { 1 }
fn default_renumber_padding() -> usize { 3 }
fn default_renumber_position() -> RenumberPosition { RenumberPosition::Prefix }

/// Configuration for the Timestamp method
/// Sets filename based on file's timestamp (created/modified/accessed)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimestampConfig {
    /// Whether this method is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Which timestamp to use
    #[serde(default = "default_timestamp_source")]
    pub source: TimestampSource,
    
    /// Date format string (chrono-compatible, e.g., "YYYY-MM-DD_HH-mm-ss")
    #[serde(default = "default_timestamp_format")]
    pub format: String,
    
    /// Which part of the filename to apply to
    pub apply_to: ApplyToOption,
}

/// Which file timestamp to use
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimestampSource {
    /// File creation time
    Created,
    /// File last modified time
    Modified,
    /// File last accessed time
    Accessed,
    /// Image EXIF original date/time
    ImgDate,
    /// Image EXIF original time
    ImgTime,
    /// Video creation date
    VidDate,
    /// Video creation time
    VidTime,
    /// Audio recording date (ID3 TDRC)
    AudDate,
    /// Audio recording time (ID3 TDRC)
    AudTime,
}

fn default_timestamp_source() -> TimestampSource { TimestampSource::Modified }
fn default_timestamp_format() -> String { "YYYY-MM-DD_HH-mm-ss".to_string() }

/// Enum representing all method configurations (used for serialization/deserialization)
/// Allows storing heterogeneous method configs in collections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MethodConfig {
    /// Replace text patterns
    Replace(ReplaceConfig),
    
    /// Insert text at positions
    Add(AddConfig),
    
    /// Remove characters
    Remove(RemoveConfig),
    
    /// Change capitalization
    NewCase(NewCaseConfig),
    
    /// Generate new names from template
    NewName(NewNameConfig),
    
    /// Rename from a list of names
    List(ListConfig),
    
    /// Move text within filename
    Move(MoveConfig),
    
    /// Trim characters from edges
    Trim(TrimConfig),
    
    /// Renumber files with sequential numbers
    Renumber(RenumberConfig),
    
    /// Set filename from file timestamp
    Timestamp(TimestampConfig),
}

/// Type identifier for each renaming method
/// Used for serialization, UI display, and method routing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MethodType {
    Replace,
    Add,
    Remove,
    NewCase,
    NewName,
    List,
    Move,
    Trim,
    Renumber,
    Timestamp,
}

impl std::fmt::Display for MethodType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MethodType::Replace => write!(f, "Replace"),
            MethodType::Add => write!(f, "Add"),
            MethodType::Remove => write!(f, "Remove"),
            MethodType::NewCase => write!(f, "NewCase"),
            MethodType::NewName => write!(f, "NewName"),
            MethodType::List => write!(f, "List"),
            MethodType::Move => write!(f, "Move"),
            MethodType::Trim => write!(f, "Trim"),
            MethodType::Renumber => write!(f, "Renumber"),
            MethodType::Timestamp => write!(f, "Timestamp"),
        }
    }
}

impl MethodConfig {
    /// Get the method type identifier for this configuration
    /// Used for routing and serialization purposes
    pub fn method_type(&self) -> MethodType {
        match self {
            MethodConfig::Replace(_) => MethodType::Replace,
            MethodConfig::Add(_) => MethodType::Add,
            MethodConfig::Remove(_) => MethodType::Remove,
            MethodConfig::NewCase(_) => MethodType::NewCase,
            MethodConfig::NewName(_) => MethodType::NewName,
            MethodConfig::List(_) => MethodType::List,
            MethodConfig::Move(_) => MethodType::Move,
            MethodConfig::Trim(_) => MethodType::Trim,
            MethodConfig::Renumber(_) => MethodType::Renumber,
            MethodConfig::Timestamp(_) => MethodType::Timestamp,
        }
    }
}
