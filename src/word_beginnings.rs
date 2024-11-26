
pub fn test_module_file() {
    println!("src/word_beginnings.rs connected");
}

pub fn remove_init_chars(word: &str, chars: usize) -> String {
    return word[chars..].to_string();
}

pub fn init_en_in_to_n(word: &str) -> String {
    if word.starts_with("en") || word.starts_with("in") {
        let end_slice = &word[2..];
        return format!("n{}", end_slice);
    }
    
    word.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_en_in_to_n() {
        assert_eq!(init_en_in_to_n("enlarge"), "nlarge".to_string());
        assert_eq!(init_en_in_to_n("endow"), "ndow".to_string());
        assert_eq!(init_en_in_to_n("inform"), "nform".to_string());
        assert_eq!(init_en_in_to_n("insist"), "nsist".to_string());
    }
    
    #[test]
    fn test_remove_init_chars() {
        assert_eq!(remove_init_chars("example", 2), "ample".to_string());
    }
}
