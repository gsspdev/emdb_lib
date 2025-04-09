use std::borrow::Cow;
use std::collections::HashMap;

use crate::Transform;

/// WordBuildingTransform is a container for all word building rules
pub struct WordBuildingTransform {
    word_sign: WordSignTransform,
    numeral: NumeralTransform,
    tense_omission: TenseOmissionTransform,
    standard_abbreviation: StandardAbbreviationTransform,
    plural_punctuation: PluralPunctuationTransform,
}

impl WordBuildingTransform {
    /// Create a new WordBuildingTransform with default settings
    pub fn new() -> Self {
        WordBuildingTransform {
            word_sign: WordSignTransform::new(),
            numeral: NumeralTransform::new(),
            tense_omission: TenseOmissionTransform::new(),
            standard_abbreviation: StandardAbbreviationTransform::new(),
            plural_punctuation: PluralPunctuationTransform::new(),
        }
    }
}

impl Transform for WordBuildingTransform {
    fn apply<'a>(&self, text: &'a str) -> Cow<'a, str> {
        if text.is_empty() {
            return Cow::Borrowed(text);
        }

        // Apply rules in order
        let result = self.word_sign.apply(text);
        let result = self.numeral.apply(&result).into_owned();
        let result = self.tense_omission.apply(&result).into_owned();
        let result = self.standard_abbreviation.apply(&result).into_owned();
        let result = self.plural_punctuation.apply(&result).into_owned();

        Cow::Owned(result)
    }
}

/// WordSignTransform implements rule 11: pre-determined abbreviations for common words
pub struct WordSignTransform {
    word_signs: HashMap<String, String>,
}

impl WordSignTransform {
    /// Create a new WordSignTransform with default word signs
    pub fn new() -> Self {
        let mut word_signs = HashMap::new();

        // Add common word signs
        word_signs.insert("therefore".to_string(), "trf".to_string());
        word_signs.insert("workplace".to_string(), "wkpl".to_string());
        word_signs.insert("daytime".to_string(), "dti".to_string());
        word_signs.insert("together".to_string(), "tgr".to_string());
        word_signs.insert("understand".to_string(), "unst".to_string());

        WordSignTransform { word_signs }
    }
}

impl Transform for WordSignTransform {
    fn apply<'a>(&self, text: &'a str) -> Cow<'a, str> {
        if text.is_empty() {
            return Cow::Borrowed(text);
        }

        // Check if this word has a pre-defined abbreviation
        if let Some(abbreviation) = self.word_signs.get(text) {
            return Cow::Owned(abbreviation.clone());
        }

        Cow::Borrowed(text)
    }
}

/// NumeralTransform implements rule 13: write numerals for numbers except "one"
pub struct NumeralTransform {
    number_words: HashMap<String, String>,
}

impl NumeralTransform {
    /// Create a new NumeralTransform
    pub fn new() -> Self {
        let mut number_words = HashMap::new();

        // Add number words conversions
        number_words.insert("two".to_string(), "2".to_string());
        number_words.insert("three".to_string(), "3".to_string());
        number_words.insert("four".to_string(), "4".to_string());
        number_words.insert("five".to_string(), "5".to_string());
        number_words.insert("six".to_string(), "6".to_string());
        number_words.insert("seven".to_string(), "7".to_string());
        number_words.insert("eight".to_string(), "8".to_string());
        number_words.insert("nine".to_string(), "9".to_string());
        number_words.insert("ten".to_string(), "10".to_string());

        NumeralTransform { number_words }
    }
}

impl Transform for NumeralTransform {
    fn apply<'a>(&self, text: &'a str) -> Cow<'a, str> {
        if text.is_empty() {
            return Cow::Borrowed(text);
        }

        // Check if this is a number word (except "one")
        if let Some(numeral) = self.number_words.get(text) {
            return Cow::Owned(numeral.clone());
        }

        Cow::Borrowed(text)
    }
}

/// TenseOmissionTransform implements rule 18: omit past tense -ed and present participle -ing
pub struct TenseOmissionTransform;

impl TenseOmissionTransform {
    /// Create a new TenseOmissionTransform
    pub fn new() -> Self {
        TenseOmissionTransform
    }
}

impl Transform for TenseOmissionTransform {
    fn apply<'a>(&self, text: &'a str) -> Cow<'a, str> {
        if text.is_empty() || text.len() < 3 {
            return Cow::Borrowed(text);
        }

        // Remove -ed from the end of words
        if text.ends_with("ed") {
            return Cow::Owned(text[..text.len() - 2].to_string());
        }

        // Remove -ing from the end of words
        if text.ends_with("ing") {
            return Cow::Owned(text[..text.len() - 3].to_string());
        }

        Cow::Borrowed(text)
    }
}

/// StandardAbbreviationTransform implements rule 35: use standard abbreviations
pub struct StandardAbbreviationTransform {
    abbreviations: HashMap<String, String>,
}

impl StandardAbbreviationTransform {
    /// Create a new StandardAbbreviationTransform
    pub fn new() -> Self {
        let mut abbreviations = HashMap::new();

        // Add standard abbreviations
        abbreviations.insert("company".to_string(), "co".to_string());
        abbreviations.insert("department".to_string(), "dept".to_string());
        abbreviations.insert("association".to_string(), "assn".to_string());
        abbreviations.insert("institute".to_string(), "inst".to_string());
        abbreviations.insert("number".to_string(), "no".to_string());
        abbreviations.insert("city".to_string(), "C".to_string());

        StandardAbbreviationTransform { abbreviations }
    }
}

impl Transform for StandardAbbreviationTransform {
    fn apply<'a>(&self, text: &'a str) -> Cow<'a, str> {
        if text.is_empty() {
            return Cow::Borrowed(text);
        }

        // Use standard abbreviations
        if let Some(abbreviation) = self.abbreviations.get(text) {
            return Cow::Owned(abbreviation.clone());
        }

        Cow::Borrowed(text)
    }
}

/// PluralPunctuationTransform implements rule 27: repeat punctuation to pluralize
pub struct PluralPunctuationTransform;

impl PluralPunctuationTransform {
    /// Create a new PluralPunctuationTransform
    pub fn new() -> Self {
        PluralPunctuationTransform
    }
}

impl Transform for PluralPunctuationTransform {
    fn apply<'a>(&self, text: &'a str) -> Cow<'a, str> {
        if text.is_empty() || text.len() < 2 {
            return Cow::Borrowed(text);
        }

        // Detect if this is a shorthand word ending with a punctuation symbol + "s"
        // For example, if we've already transformed "card" to "k/"
        // and we see "k/s", we should transform it to "k//"
        let last_char = text.chars().last().unwrap();

        if last_char == 's' {
            let prefix = &text[..text.len() - 1];
            if !prefix.is_empty() {
                let prefix_last_char = prefix.chars().last().unwrap();
                if matches!(prefix_last_char, '/' | '-' | '\'') {
                    let result = format!("{}{}", prefix, prefix_last_char);
                    return Cow::Owned(result);
                }
            }
        }

        Cow::Borrowed(text)
    }
}

#[allow(dead_code)]
pub fn test_module_file() {
    println!("words/building.rs connected");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_sign_transform() {
        let transform = WordSignTransform::new();
        assert_eq!(transform.apply("therefore").into_owned(), "trf");
        assert_eq!(transform.apply("together").into_owned(), "tgr");
        // Non-matching word should remain unchanged
        assert_eq!(transform.apply("normal").into_owned(), "normal");
    }

    #[test]
    fn test_numeral_transform() {
        let transform = NumeralTransform::new();
        assert_eq!(transform.apply("two").into_owned(), "2");
        assert_eq!(transform.apply("three").into_owned(), "3");
        assert_eq!(transform.apply("one").into_owned(), "one"); // "one" should remain unchanged
    }

    #[test]
    fn test_tense_omission_transform() {
        let transform = TenseOmissionTransform::new();
        assert_eq!(transform.apply("worked").into_owned(), "work");
        assert_eq!(transform.apply("working").into_owned(), "work");
        assert_eq!(transform.apply("normal").into_owned(), "normal");
    }

    #[test]
    fn test_standard_abbreviation_transform() {
        let transform = StandardAbbreviationTransform::new();
        assert_eq!(transform.apply("company").into_owned(), "co");
        assert_eq!(transform.apply("department").into_owned(), "dept");
        assert_eq!(transform.apply("normal").into_owned(), "normal");
    }

    #[test]
    fn test_plural_punctuation_transform() {
        let transform = PluralPunctuationTransform::new();
        assert_eq!(transform.apply("k/s").into_owned(), "k//");
        assert_eq!(transform.apply("t-s").into_owned(), "t--");
        assert_eq!(transform.apply("normal").into_owned(), "normal");
    }
}
