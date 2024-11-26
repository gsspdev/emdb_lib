pub mod phrasing;
pub mod punctuation;
pub mod phonetics;
pub mod phoneme_clusters;
pub mod words;
pub mod types;
pub mod suffixes;
//use crate::words::*;
// use crate::building::*;
// use crate::beginnings::*;
// use crate::endings_additional::*;
// use crate::endings_freq::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::words::*;

    #[test]
    fn test_modules_connected() {
        phrasing::test_module_file();
        punctuation::test_module_file();
        phonetics::test_module_file();
        phoneme_clusters::test_module_file();
        beginnings::test_module_file();
        building::test_module_file();
        endings_freq::test_module_file();
        endings_additional::test_module_file();
    }
}
