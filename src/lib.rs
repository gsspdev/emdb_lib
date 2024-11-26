mod phrasing;
mod punctuation;
mod phonetics;
mod phoneme_clusters;
mod word_building;
mod word_beginnings;
mod word_endings_additional;
mod word_endings_freq;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modules_connected() {
        phrasing::test_module_file();
        punctuation::test_module_file();
        phonetics::test_module_file();
        phoneme_clusters::test_module_file();
        word_beginnings::test_module_file();
        word_building::test_module_file();
        word_endings_freq::test_module_file();
        word_endings_additional::test_module_file();
    }
}
