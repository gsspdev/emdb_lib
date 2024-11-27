pub fn test_module_file() {
    println!("morphemes/prefixes.rs connected");
}

pub struct PrefixList {
    prefixes: &'static [&'static str],
}

impl PrefixList {
    pub fn new() -> Self {
        Self {
            prefixes: LIST_OF_PREFIXES,
        }
    }

    pub fn return_prefixes(&self) -> &'static [&'static str] {
       return self.prefixes
    }

    pub fn print_prefix_entries(&self) {
        println!("///////////////////////");
        println!("Printing prefixes:");
        for entry in self.prefixes {
            println!("{}", entry);
        }
        println!("///////////////////////");
    }
}

static LIST_OF_PREFIXES: &[&str] = &[
    "a", "after", "back", "be", "by", "down", "en", "fore", "hind", "mid", "midi", "mini", "mis",
    "off", "on", "out", "over", "self", "step", "twi", "un", "un", "under", "up", "with", "a",
    "Afro", "ambi", "amphi", "an", "ana", "Anglo", "ante", "anti", "apo", "arch", "astro", "auto",
    "bi", "bio", "circum", "cis", "con", "contra", "counter", "cryo", "crypto", "de", "demi",
    "demo", "deuter", "di", "dia", "dis", "du", "eco", "electro", "en", "epi", "eu", "Euro", "ex",
    "extra", "Franco", "geo", "gyro", "hetero", "hemi", "Hispano", "homo", "hydro", "hyper",
    "hypo", "ideo", "idio", "in", "Indo", "in", "infra", "inter", "intra", "iso", "Italo", "macro",
    "mal", "maxi", "mega", "meso", "meta", "micro", "mono", "multi", "neo", "non", "ob", "omni",
    "ortho", "paleo", "pan", "para", "ped", "pen", "per", "peri", "photo", "pleo", "pod", "poly",
    "post", "pre", "preter", "pro", "pro", "pros", "proto", "pseudo", "pyro", "quadri", "quasi",
    "retro", "semi", "socio", "sub", "super", "supra", "sur", "syn", "tele", "trans", "tri",
    "ultra", "uni", "vice", "gain", "umbe", "y",
];

#[cfg(test)]

#[test]
pub fn returns_prefixes() {
    let pl = PrefixList::new();
    let prefixes_from_return_prefixes = pl.return_prefixes();
    assert_eq!(prefixes_from_return_prefixes, LIST_OF_PREFIXES);
    
//    println!("prefixes_from_return_prefixes: {:?}", prefixes_from_return_prefixes);
//    println!("IS EQUAL TO:");
//    println!("LIST_OF_PREFIXES: {:?}", LIST_OF_PREFIXES);
}
