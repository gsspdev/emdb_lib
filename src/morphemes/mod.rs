pub mod prefixes;
pub mod suffixes;

/// A trait for efficient morpheme lookup
pub trait MorphemeLookup {
    /// Check if a string starts with any of the morphemes
    #[allow(dead_code)]
    fn starts_with_any(&self, text: &str) -> Option<&str>;

    /// Check if a string ends with any of the morphemes
    #[allow(dead_code)]
    fn ends_with_any(&self, text: &str) -> Option<&str>;
}
