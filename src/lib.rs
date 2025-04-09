mod morphemes;
mod phoneme_clusters;
mod phonetics;
mod phrasing;
mod punctuation;
mod types;
mod utilities;
pub mod words;

use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Error type for speedwriting operations
#[derive(Debug)]
pub enum SpeedwritingError {
    /// Input text is invalid for the requested operation
    InvalidInput(String),
    /// Failed to apply a transformation
    TransformationFailed(String),
}

impl Display for SpeedwritingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            SpeedwritingError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            SpeedwritingError::TransformationFailed(msg) => {
                write!(f, "Transformation failed: {}", msg)
            }
        }
    }
}

impl Error for SpeedwritingError {}

/// Result type for speedwriting operations
pub type Result<T> = std::result::Result<T, SpeedwritingError>;

/// Trait for text transformations
pub trait Transform {
    /// Apply the transformation to the input text
    fn apply<'a>(&self, text: &'a str) -> Cow<'a, str>;
}

/// Main API for Speedwriting
///
/// This struct provides methods to convert text to and from Dearborn's
/// Speedwriting notation, based on the 108 principles of Speedwriting.
///
/// # Examples
///
/// ```
/// use emdb_lib::Speedwriter;
///
/// // Create a new Speedwriter with default settings
/// let speedwriter = Speedwriter::new();
///
/// // Convert text to speedwriting notation
/// let result = speedwriter.to_speedwriting("encode this message").unwrap();
/// ```
pub struct Speedwriter {
    transforms: Vec<Box<dyn Transform>>,
}

impl Default for Speedwriter {
    fn default() -> Self {
        Self::new()
    }
}

impl Speedwriter {
    /// Create a new Speedwriter with default settings
    pub fn new() -> Self {
        let mut transforms: Vec<Box<dyn Transform>> = Vec::new();

        // Add transforms in the correct order based on the 108 principles
        transforms.push(Box::new(words::beginnings::PrefixTransform::new()));
        transforms.push(Box::new(words::endings_freq::SuffixTransform::new()));
        transforms.push(Box::new(words::building::WordBuildingTransform::new()));
        transforms.push(Box::new(phoneme_clusters::PhonemeClusterTransform::new()));

        Speedwriter { transforms }
    }

    /// Convert text to speedwriting notation
    ///
    /// This method applies all the Speedwriting principles to the input text,
    /// resulting in a compressed, phonetic representation.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to convert
    ///
    /// # Returns
    ///
    /// * `Result<String>` - The converted text or an error
    pub fn to_speedwriting(&self, text: &str) -> Result<String> {
        if text.is_empty() {
            return Ok(String::new());
        }

        // Process word by word to preserve spacing
        let words: Vec<&str> = text.split_whitespace().collect();

        let transformed_words: Vec<String> = words
            .iter()
            .map(|&word| self.transform_word(word).to_string())
            .collect();

        Ok(transformed_words.join(" "))
    }

    /// Convert speedwriting notation back to regular text
    ///
    /// # Arguments
    ///
    /// * `text` - The speedwriting notation to convert
    ///
    /// # Returns
    ///
    /// * `Result<String>` - The converted text or an error
    pub fn from_speedwriting(&self, _text: &str) -> Result<String> {
        // Not implemented yet - would require reverse transformations
        Err(SpeedwritingError::TransformationFailed(
            "From-speedwriting conversion not implemented yet".to_string(),
        ))
    }

    /// Apply all transformations to a single word
    fn transform_word<'a>(&self, word: &'a str) -> Cow<'a, str> {
        if word.is_empty() {
            return Cow::Borrowed(word);
        }

        let mut result = String::from(word);

        // Apply all transforms in order
        for transform in &self.transforms {
            // Use owned string to avoid borrowing issues
            let temp = transform.apply(&result).into_owned();
            result = temp;
        }

        Cow::Owned(result)
    }
}

/// Builder for creating customized Speedwriter instances
pub struct SpeedwriterBuilder {
    transforms: Vec<Box<dyn Transform>>,
}

impl SpeedwriterBuilder {
    /// Create a new SpeedwriterBuilder
    pub fn new() -> Self {
        SpeedwriterBuilder {
            transforms: Vec::new(),
        }
    }

    /// Add a transformation rule
    pub fn with_rule<T: Transform + 'static>(mut self, rule: T) -> Self {
        self.transforms.push(Box::new(rule));
        self
    }

    /// Add all default rules
    pub fn with_default_rules(mut self) -> Self {
        self.transforms
            .push(Box::new(words::beginnings::PrefixTransform::new()));
        self.transforms
            .push(Box::new(words::endings_freq::SuffixTransform::new()));
        self.transforms
            .push(Box::new(words::building::WordBuildingTransform::new()));
        self.transforms
            .push(Box::new(phoneme_clusters::PhonemeClusterTransform::new()));
        self
    }

    /// Add only word building rules
    pub fn with_word_building_rules(mut self) -> Self {
        self.transforms
            .push(Box::new(words::building::WordSignTransform::new()));
        self.transforms
            .push(Box::new(words::building::NumeralTransform::new()));
        self.transforms
            .push(Box::new(words::building::TenseOmissionTransform::new()));
        self.transforms.push(Box::new(
            words::building::StandardAbbreviationTransform::new(),
        ));
        self.transforms
            .push(Box::new(words::building::PluralPunctuationTransform::new()));
        self
    }

    /// Build the Speedwriter
    pub fn build(self) -> Speedwriter {
        Speedwriter {
            transforms: self.transforms,
        }
    }
}

impl Speedwriter {
    /// Create a new SpeedwriterBuilder
    ///
    /// This allows for customizing which rules are applied
    ///
    /// # Examples
    ///
    /// ```
    /// use emdb_lib::Speedwriter;
    ///
    /// let speedwriter = Speedwriter::builder()
    ///     .with_default_rules()
    ///     .build();
    /// ```
    pub fn builder() -> SpeedwriterBuilder {
        SpeedwriterBuilder::new()
    }
}

// Public exports
pub use morphemes::prefixes::PrefixList;
pub use morphemes::suffixes::SuffixList;
pub use phoneme_clusters::PhonemeClusterTransform;
pub use words::beginnings::PrefixTransform;
pub use words::building::{
    NumeralTransform, PluralPunctuationTransform, StandardAbbreviationTransform,
    TenseOmissionTransform, WordBuildingTransform, WordSignTransform,
};
pub use words::endings_freq::SuffixTransform;
