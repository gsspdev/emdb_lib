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

//## Punctuation
//
//- (5) A declarative sentence ends with a period. (There is no need to type a space after the period.)
//- (10) An interrogative sentence ends with a question mark. (There is no need to type a space after the question mark.)
//- (62) Indicate the end of a paragraph by repeating the period or question mark at the end of the final sentence.
//- (98) To indicate a correction or addition within a text, use three diagonal slashes ( /// ) at the beginning of the new or corrected material and three slashes at the end of it.
//
//Note: The first letter of a sentence is not automatically capitalized in Speedwriting. Upper-case letters are reserved for use as symbols that represent various prefixes, suffixes and sound-clusters.
//Note: In most shorthand systems, commas are almost never used. Context makes it clear where commas would be inserted when transcribi