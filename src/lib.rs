mod morphemes;
mod phoneme_clusters;
mod phonetics;
mod phrasing;
mod punctuation;
mod types;
mod utilities;
mod words;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::morphemes::prefixes::PrefixList;
    use crate::morphemes::suffixes::SuffixList;
    use crate::morphemes::{prefixes, suffixes};
    use crate::words::{beginnings, building, endings_additional, endings_freq};

    #[test]
    fn test_modules_connected() {
        phoneme_clusters::test_module_file();
        phonetics::test_module_file();
        phrasing::test_module_file();
        prefixes::test_module_file();
        punctuation::test_module_file();
        suffixes::test_module_file();
        types::test_module_file();
        // words/
        beginnings::test_module_file();
        building::test_module_file();
        endings_additional::test_module_file();
        endings_freq::test_module_file();
    }

    #[test]
    fn test_print_suffix_entries() {
        let suffix_list = SuffixList::new();
        suffix_list.print_suffix_entries();
    }

    #[test]
    fn test_print_prefix_entries() {
        let prefix_list = PrefixList::new();
        prefix_list.print_prefix_entries();
    }
}
