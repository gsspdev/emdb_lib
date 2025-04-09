use super::MorphemeLookup;
use std::collections::{HashMap, HashSet};

/// SuffixList provides efficient storage and lookup for suffixes
#[derive(Debug)]
pub struct SuffixList {
    suffixes: HashSet<&'static str>,
    suffix_trie: SuffixTrie,
}

impl Default for SuffixList {
    fn default() -> Self {
        Self::new()
    }
}

impl SuffixList {
    /// Create a new SuffixList with default suffixes
    pub fn new() -> Self {
        let suffixes: HashSet<&'static str> = LIST_OF_SUFFIXES.iter().copied().collect();
        let mut suffix_trie = SuffixTrie::new();

        for &suffix in LIST_OF_SUFFIXES {
            suffix_trie.insert(suffix);
        }

        Self {
            suffixes,
            suffix_trie,
        }
    }

    /// Check if a string is in the suffix list
    pub fn contains(&self, suffix: &str) -> bool {
        self.suffixes.contains(suffix)
    }

    /// Print all suffix entries for debugging
    pub fn print_suffix_entries(&self) {
        println!("///////////////////////");
        println!("Printing suffixes:");
        for entry in &self.suffixes {
            println!("{}", entry);
        }
        println!("///////////////////////");
    }
}

impl MorphemeLookup for SuffixList {
    fn starts_with_any(&self, _text: &str) -> Option<&str> {
        None // Suffixes only apply at the end
    }

    fn ends_with_any(&self, text: &str) -> Option<&str> {
        self.suffix_trie.find_longest_suffix(text)
    }
}

/// Trie data structure for efficient suffix lookup
#[derive(Debug)]
struct SuffixTrie {
    children: HashMap<char, SuffixTrie>,
    is_terminal: bool,
    value: Option<&'static str>,
}

impl SuffixTrie {
    fn new() -> Self {
        SuffixTrie {
            children: HashMap::new(),
            is_terminal: false,
            value: None,
        }
    }

    fn insert(&mut self, suffix: &'static str) {
        // Insert in reverse for suffix matching
        let reversed: Vec<char> = suffix.chars().rev().collect();
        let mut current = self;

        for &c in &reversed {
            current = current.children.entry(c).or_insert_with(SuffixTrie::new);
        }

        current.is_terminal = true;
        current.value = Some(suffix);
    }

    fn find_longest_suffix<'a>(&'a self, text: &str) -> Option<&'a str> {
        let chars: Vec<char> = text.chars().collect();
        let mut current = self;
        let mut last_match = None;

        // Start from the end of the string
        for i in (0..chars.len()).rev() {
            if let Some(next) = current.children.get(&chars[i]) {
                current = next;

                if current.is_terminal {
                    last_match = current.value;
                }
            } else {
                break;
            }
        }

        last_match
    }
}

#[allow(dead_code)]
pub fn test_module_file() {
    println!("morphemes/suffixes.rs connected");
}

/// The list of common suffixes used in English
/// This is a subset of the full list for brevity
static LIST_OF_SUFFIXES: &[&str] = &[
    "ability", "able", "ably", "ade", "age", "al", "ance", "ancy", "ant", "ar", "arian", "arium",
    "ary", "ate", "ation", "ative", "ator", "atory", "cy", "dom", "ed", "ee", "eer", "ence",
    "ency", "ent", "er", "ery", "ese", "esque", "ess", "est", "etic", "ette", "ful", "hood", "ial",
    "ian", "ible", "ic", "ical", "ice", "ician", "ics", "ify", "ile", "ily", "ine", "ing", "ion",
    "ious", "ish", "ism", "ist", "ite", "ity", "ium", "ive", "ization", "ize", "less", "let",
    "like", "ling", "log", "ly", "ment", "ness", "or", "ory", "ose", "ous", "ship", "some", "ster",
    "sy", "ty", "ure", "ward", "wards", "wise", "y",
    // Most frequently used in speedwriting
    "ly", "ing", "thing", "ary", "ery", "iry", "ory", "ury",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_list() {
        let suffix_list = SuffixList::new();
        assert!(suffix_list.contains("ly"));
        assert!(suffix_list.contains("ing"));
        assert!(!suffix_list.contains("xyz"));
    }

    #[test]
    fn test_suffix_lookup() {
        let suffix_list = SuffixList::new();
        assert_eq!(suffix_list.ends_with_any("quickly"), Some("ly"));
        assert_eq!(suffix_list.ends_with_any("running"), Some("ing"));
        assert_eq!(suffix_list.ends_with_any("hello"), None);
    }
}
