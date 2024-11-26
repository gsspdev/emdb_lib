use regex::Regex;

pub fn test_module_file() {
    println!("src/punctuation.rs connected");
}

pub fn substitute_punctuation(content: &str) -> String {
    // Define the regex pattern to match punctuation at the end of paragraphs
    let content = content;
    let re = Regex::new(r"([.!?])\s*(\n\s*\n|\n*\z)").unwrap();

    // Substitution to double the punctuation marks
    let result = re.replace_all(&content, "$1$1$2");
    result.to_string()
}
