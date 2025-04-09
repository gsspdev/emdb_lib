use super::MorphemeLookup;
use std::collections::{HashMap, HashSet};

/// PrefixList provides efficient storage and lookup for prefixes
#[derive(Debug)]
pub struct PrefixList {
    prefixes: HashSet<&'static str>,
    prefix_trie: PrefixTrie,
}

impl Default for PrefixList {
    fn default() -> Self {
        Self::new()
    }
}

impl PrefixList {
    /// Create a new PrefixList with default prefixes
    pub fn new() -> Self {
        let prefixes: HashSet<&'static str> = LIST_OF_PREFIXES.iter().copied().collect();
        let mut prefix_trie = PrefixTrie::new();

        for &prefix in LIST_OF_PREFIXES {
            prefix_trie.insert(prefix);
        }

        Self {
            prefixes,
            prefix_trie,
        }
    }

    /// Get the list of all prefixes
    pub fn return_prefixes(&self) -> &'static [&'static str] {
        LIST_OF_PREFIXES
    }

    /// Check if a string is in the prefix list
    pub fn contains(&self, prefix: &str) -> bool {
        self.prefixes.contains(prefix)
    }

    /// Print all prefix entries for debugging
    pub fn print_prefix_entries(&self) {
        println!("///////////////////////");
        println!("Printing prefixes:");
        for entry in &self.prefixes {
            println!("{}", entry);
        }
        println!("///////////////////////");
    }
}

impl MorphemeLookup for PrefixList {
    fn starts_with_any(&self, text: &str) -> Option<&str> {
        self.prefix_trie.find_longest_prefix(text)
    }

    fn ends_with_any(&self, _text: &str) -> Option<&str> {
        None // Prefixes only apply at the start
    }
}

/// Trie data structure for efficient prefix lookup
#[derive(Debug)]
struct PrefixTrie {
    children: HashMap<char, PrefixTrie>,
    is_terminal: bool,
    value: Option<&'static str>,
}

impl PrefixTrie {
    fn new() -> Self {
        PrefixTrie {
            children: HashMap::new(),
            is_terminal: false,
            value: None,
        }
    }

    fn insert(&mut self, prefix: &'static str) {
        let mut current = self;

        for c in prefix.chars() {
            current = current.children.entry(c).or_insert_with(PrefixTrie::new);
        }

        current.is_terminal = true;
        current.value = Some(prefix);
    }

    fn find_longest_prefix<'a>(&'a self, text: &str) -> Option<&'a str> {
        let mut current = self;
        let mut last_match = None;

        for (_, c) in text.chars().enumerate() {
            if let Some(next) = current.children.get(&c) {
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
    println!("morphemes/prefixes.rs connected");
}

/// The list of common prefixes used in English
static LIST_OF_PREFIXES: &[&str] = &[
    "a", "after", "back", "be", "by", "down", "en", "fore", "hind", "mid", "midi", "mini", "mis",
    "off", "on", "out", "over", "self", "step", "twi", "un", "under", "up", "with", "a", "Afro",
    "ambi", "amphi", "an", "ana", "Anglo", "ante", "anti", "apo", "arch", "astro", "auto", "bi",
    "bio", "circum", "cis", "con", "contra", "counter", "cryo", "crypto", "de", "demi", "demo",
    "deuter", "di", "dia", "dis", "du", "eco", "electro", "en", "epi", "eu", "Euro", "ex", "extra",
    "Franco", "geo", "gyro", "hetero", "hemi", "Hispano", "homo", "hydro", "hyper", "hypo", "ideo",
    "idio", "in", "Indo", "in", "infra", "inter", "intra", "iso", "Italo", "macro", "mal", "maxi",
    "mega", "meso", "meta", "micro", "mono", "multi", "neo", "non", "ob", "omni", "ortho", "paleo",
    "pan", "para", "ped", "pen", "per", "peri", "photo", "pleo", "pod", "poly", "post", "pre",
    "preter", "pro", "pros", "proto", "pseudo", "pyro", "quadri", "quasi", "retro", "semi",
    "socio", "sub", "super", "supra", "sur", "syn", "tele", "trans", "tri", "ultra", "uni", "vice",
    "gain", "umbe", "y",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_list() {
        let prefix_list = PrefixList::new();
        assert!(prefix_list.contains("un"));
        assert!(prefix_list.contains("pre"));
        assert!(!prefix_list.contains("xyz"));
    }

    #[test]
    fn test_prefix_lookup() {
        let prefix_list = PrefixList::new();
        assert_eq!(prefix_list.starts_with_any("unpacked"), Some("un"));
        assert_eq!(prefix_list.starts_with_any("predefined"), Some("pre"));
        assert_eq!(prefix_list.starts_with_any("hello"), None);
    }
}
