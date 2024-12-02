// use std::ops::Deref;
// use str_utils::*;

#[test]
fn test_replace_prefix() {
    let original = "example";
    let result = original;
    assert_eq!(result, "nample");
}

// (40) Write the letter n to express initial en-, in-
pub fn init_en_in_to_n(word: &str) -> String {
    match word {
        w if w.starts_with("en") => word.replacen("en", "n", 1),
        w if w.starts_with("in") => word.replacen("in", "n", 1),
        _ => word.to_string(),
    }
}

#[test]
fn test_init_en_in_to_n() {
    assert_eq!(init_en_in_to_n("unmodified"), "unmodified");
    assert_eq!(init_en_in_to_n("enlarge"), "nlarge");
    assert_eq!(init_en_in_to_n("endow"), "ndow");
    assert_eq!(init_en_in_to_n("inform"), "nform");
    assert_eq!(init_en_in_to_n("insist"), "nsist");
}

// (46) Write the letter u to express “un” at the beginning of a word.
pub fn init_un_to_u(word: &str) -> String {
    match word {
        w if w.starts_with("un") => word.replacen("un", "u", 1),
        _ => word.to_string(),
    }
}

#[test]
fn test_init_un_to_u() {
    assert_eq!(init_un_to_u("tester"), "tester");
    assert_eq!(init_un_to_u("unwise"), "uwise".to_string());
}

// (75) Write i to express initial im-. This Principle can also be used when there are prefixes added to word that started out with im-, but this Principle does not apply to words like “crimson” or “simmer” that just happen to contatin the sequence I+M.
pub fn init_im_to_i(word: &str) -> String {
    match word {
        w if w.starts_with("im") && w.chars().nth(2).map_or(false, |c| c == 'a' || c == 'i') => {
            let without_im = w.replacen("im", "i", 1);
            format!("{}{}", &without_im[0..1], &without_im[2..])
        }
        w if w.starts_with("im") => w.replacen("im", "i", 1),
        _ => word.to_string(),
    }
}

#[test]
fn test_init_im_to_i() {
    assert_eq!(init_im_to_i("unmodified"), "unmodified");
    assert_eq!(init_im_to_i("image"), "ige");
    assert_eq!(init_im_to_i("imitate"), "itate");
    assert_eq!(init_im_to_i("imply"), "iply");
    //        assert_eq!(init_im_to_in("unimpaired"), "unpaired".to_string());
    //        assert_eq!(init_im_to_in("unimproved"), "unproved".to_string());
}

// (89) Write m to express initial and medial em or um
pub fn em_um_to_m(word: &str) -> String {
    match word {
        w if !w.ends_with("em") && w.contains("em") => w.replace("em", "m"),
        w if !w.ends_with("um") && w.contains("um") => w.replace("um", "m"),
        _ => word.to_string(),
    }
}

#[test]
fn test_em_um_to_m() {
    assert_eq!(em_um_to_m("unmodified"), "unmodified");
    assert_eq!(em_um_to_m("emphasize"), "mphasize");
    assert_eq!(em_um_to_m("empty"), "mpty");
    assert_eq!(em_um_to_m("umpire"), "mpire");
    assert_eq!(em_um_to_m("umbilical"), "mbilical");
}

// (42) Write a to express initial ar-, e for er-, o for or-, u for ur-. Apparently this Principle only applies to words in which the vowel and the r are members of the same syllable. For exmaple, you cannot use this Principle to write “arise” or “erase.”
pub fn init_vowel_r_to_r(word: &str) -> String {
    //let mut word = word.clone();
    match word {
        w if w.starts_with("ar") => w.replacen("ar", "a", 1),
        w if w.starts_with("er") => w.replacen("er", "e", 1),
        w if w.starts_with("ir") => w.replacen("ir", "i", 1),
        w if w.starts_with("or") => w.replacen("or", "o", 1),
        w if w.starts_with("ur") => w.replacen("ur", "u", 1),
        w if w.starts_with("ear") => w.replacen("ear", "e", 1),
        _ => word.to_string(),
    }
}

#[test]
fn test_init_vowel_r_to_r() {
    assert_eq!(init_vowel_r_to_r("arch"), "ach");
    assert_eq!(init_vowel_r_to_r("earth"), "eth");
    assert_eq!(init_vowel_r_to_r("origin"), "oigin");
    assert_eq!(init_vowel_r_to_r("urban"), "uban");
}

//(56) Omit the vowels in be-, de-, di-, dis-, mis-, re-. Although it is not mentioned in the textbook, the dictionary indicates that the vowel is also omitted in bi-.
pub fn omit_init_vowel(word: &str) -> String {
    match word {
        w if w.starts_with("be") => w.replacen("be", "b", 1),
        w if w.starts_with("de") => w.replacen("de", "d", 1),
        w if w.starts_with("dis") => w.replacen("dis", "ds", 1),
        w if w.starts_with("di") => w.replacen("di", "d", 1),
        w if w.starts_with("mis") => w.replacen("mis", "ms", 1),
        w if w.starts_with("re") => w.replacen("re", "r", 1),
        w if w.starts_with("bi") => w.replacen("bi", "b", 1),
        _ => word.to_string(),
    }
}

#[test]
fn test_omit_init_vowel() {
    assert_eq!(omit_init_vowel("beside"), "bside");
    assert_eq!(omit_init_vowel("biped"), "bped");
    assert_eq!(omit_init_vowel("detect"), "dtect");
    assert_eq!(omit_init_vowel("direct"), "drect");
    assert_eq!(omit_init_vowel("dispel"), "dspel");
    assert_eq!(omit_init_vowel("misjudge"), "msjudge");
    assert_eq!(omit_init_vowel("remove"), "rmove");
    assert_eq!(omit_init_vowel("belated"), "blated");
    assert_eq!(omit_init_vowel("bicycle"), "bcycle");
    assert_eq!(omit_init_vowel("detachment"), "dtachment");
    assert_eq!(omit_init_vowel("diurnal"), "durnal");
    assert_eq!(omit_init_vowel("disinfect"), "dsinfect");
    assert_eq!(omit_init_vowel("misunderstand"), "msunderstand");
    assert_eq!(omit_init_vowel("reduction"), "rduction");
}

//(69) Write nc to express the word-beginnings encli-, enclo-, incle-, incli-, inclo-, inclu-.
pub fn nc_shortcut(word: &str) -> String {
    let return_string: String;
    match word {
        w if w.starts_with("encli") => return_string = format!("nc{}", &word[5..]).to_string(),
        w if w.starts_with("enclo") => return_string = format!("nc{}", &word[5..]).to_string(),
        w if w.starts_with("incle") => return_string = format!("nc{}", &word[5..]).to_string(),
        w if w.starts_with("incli") => return_string = format!("nc{}", &word[5..]).to_string(),
        w if w.starts_with("inclo") => return_string = format!("nc{}", &word[5..]).to_string(),
        w if w.starts_with("inclu") => return_string = format!("nc{}", &word[5..]).to_string(),
        _ => return word.to_string(),
    }
    return_string.to_string()
}

#[test]
fn test_nc_shortcut() {
    assert_eq!(nc_shortcut("enclitic"), "nctic");
    assert_eq!(nc_shortcut("enclosure"), "ncsure");
    assert_eq!(nc_shortcut("inclement"), "ncment");
    assert_eq!(nc_shortcut("inclination"), "ncnation");
    assert_eq!(nc_shortcut("inclose"), "ncse");
    assert_eq!(nc_shortcut("include"), "ncde");
}

//(70) Write dc to express the word-beginnings decla-, decle-, decli-.
pub fn dc_shortcut(word: &str) -> String {
    let return_string: String;
    match word {
        w if w.starts_with("decla") => return_string = format!("dc{}", &word[5..]).to_string(),
        w if w.starts_with("decle") => return_string = format!("dc{}", &word[5..]).to_string(),
        w if w.starts_with("decli") => return_string = format!("dc{}", &word[5..]).to_string(),
        w if w.starts_with("decor") => return_string = format!("dc{}", &word[5..]).to_string(),
        _ => return word.to_string(),
    }
    return_string.to_string()
}

#[test]
fn test_dc_shortcut() {
    assert_eq!(dc_shortcut("decorative"), "dcative");
    assert_eq!(dc_shortcut("declension"), "dcnsion");
    assert_eq!(dc_shortcut("decline"), "dcne");
}

//(71) Write rc to express the word-beginnings recla-, recli-, reclu-.
pub fn rc_shortcut(word: &str) -> String {
    let return_string: String;
    match word {
        w if w.starts_with("recla") => return_string = format!("rc{}", &word[5..]).to_string(),
        w if w.starts_with("recli") => return_string = format!("rc{}", &word[5..]).to_string(),
        w if w.starts_with("reclu") => return_string = format!("rc{}", &word[5..]).to_string(),
        _ => return word.to_string(),
    }
    return_string.to_string()
}

#[test]
fn test_rc_shortcut() {
    assert_eq!(rc_shortcut("reclaim"), "rcim");
    assert_eq!(rc_shortcut("recline"), "rcne");
    assert_eq!(rc_shortcut("reclusive"), "rcsive");
}

//(63) Use upper-case D to express deter- or detri-
pub fn detxx_to_upper_d(word: &str) -> String {
    let return_string: String;
    match word {
        w if w.starts_with("deter") => return_string = format!("D{}", &word[5..]).to_string(),
        w if w.starts_with("detri") => return_string = format!("D{}", &word[5..]).to_string(),
        _ => return word.to_string(),
    }
    return_string.to_string()
}

#[test]
pub fn test_detxx_to_upper_d() {
    assert_eq!(detxx_to_upper_d("determine"), "Dmine");
    assert_eq!(detxx_to_upper_d("detrimental"), "Dmental");
}

//(64) Write Al to express initial or medial alter-
pub fn alter_to_upper_al(word: &str) -> String {
    match word {
        w if !w.ends_with("alter") && w.contains("alter") => w.replace("alter", "Al"),
        _ => word.to_string(),
    }
}

#[test]
pub fn test_alter_to_al() {
    assert_eq!(alter_to_upper_al("alternative"), "Alnative");
    assert_eq!(alter_to_upper_al("alteration"), "Alation");
}

//(66) Pa represents initial pater-, patri-, patro-
pub fn patxx_to_upper_pa(word: &str) -> String {
    match word {
        w if w.starts_with("pater") => return format!("Pa{}", &word[5..]).to_string(),
        w if w.starts_with("patri") => return format!("Pa{}", &word[5..]).to_string(),
        w if w.starts_with("patro") => return format!("Pa{}", &word[5..]).to_string(),
        _ => return word.to_string(),
    }
}

#[test]
pub fn test_patxx_to_upper_pa() {
    assert_eq!(patxx_to_upper_pa("paternity"), "Panity");
    assert_eq!(patxx_to_upper_pa("patriot"), "Paot");
    assert_eq!(patxx_to_upper_pa("patrony"), "Pany");
    assert_eq!(patxx_to_upper_pa("unchanged"), "unchanged")
}

//(85) Write upper-case T to express the prefix trans-
pub fn trans_to_upper_t(word: &str) -> String {
    match word {
        w if w.starts_with("trans") => w.replace("trans", "T"),
        _ => word.to_string(),
    }
}

#[test]
pub fn test_trans_to_upper_t() {
    assert_eq!(trans_to_upper_t("transfer"), "Tfer");
    assert_eq!(trans_to_upper_t("transmutable"), "Tmutable");
    assert_eq!(trans_to_upper_t("unchanged"), "unchanged")
}

//(88) Write upper-case X to express the word-beginnings exter-, extir-, extra-, extre-, extri-, extru-.
pub fn extxx_to_upper_x(word: &str) -> String {
    let return_string: String;
    match word {
        w if w.starts_with("exter") => return_string = format!("X{}", &word[5..]).to_string(),
        w if w.starts_with("extir") => return_string = format!("X{}", &word[5..]).to_string(),
        w if w.starts_with("extra") => return_string = format!("X{}", &word[5..]).to_string(),
        w if w.starts_with("extre") => return_string = format!("X{}", &word[5..]).to_string(),
        w if w.starts_with("extri") => return_string = format!("X{}", &word[5..]).to_string(),
        w if w.starts_with("extru") => return_string = format!("X{}", &word[5..]).to_string(),
        _ => return word.to_string(),
    }
    return_string.to_string()
}

#[test]
pub fn test_extxx_to_upper_x() {
    assert_eq!(extxx_to_upper_x("external"), "Xnal");
    assert_eq!(extxx_to_upper_x("extract"), "Xct");
    assert_eq!(extxx_to_upper_x("extreme"), "Xme");
    assert_eq!(extxx_to_upper_x("extricate"), "Xcate");
    assert_eq!(extxx_to_upper_x("extrude"), "Xde");
    assert_eq!(extxx_to_upper_x("unchanged"), "unchanged");
}

//(96) Write upper-case R to express the word-beginnings retre-, retri-, retro-
pub fn retxx_to_upper_r(word: &str) -> String {
    let return_string: String;
    match word {
        w if w.starts_with("retre") => return_string = format!("R{}", &word[5..]).to_string(),
        w if w.starts_with("retri") => return_string = format!("R{}", &word[5..]).to_string(),
        w if w.starts_with("retro") => return_string = format!("R{}", &word[5..]).to_string(),
        _ => return word.to_string(),
    }
    return_string.to_string()
}

#[test]
pub fn test_retxx_to_upper_r() {
    assert_eq!(retxx_to_upper_r("retreat"), "Rat");
    assert_eq!(retxx_to_upper_r("retribution"), "Rbution");
    assert_eq!(retxx_to_upper_r("retrospective"), "Rspective");
    assert_eq!(retxx_to_upper_r("unchanged"), "unchanged");
}

//(102) Write upper-case A to express the word-beginnings attra-, attri-, atro-
pub fn atxrx_to_upper_a(word: &str) -> String {
    match word {
        w if w.starts_with("attra") => format!("A{}", &word[5..]).to_string(),
        w if w.starts_with("attri") => format!("A{}", &word[5..]).to_string(),
        w if w.starts_with("atro") => format!("A{}", &word[4..]).to_string(),
        _ => return word.to_string(),
    }
}

#[test]
pub fn test_atxrx_to_upper_a() {
    assert_eq!(atxrx_to_upper_a("attractive"), "Active");
    assert_eq!(atxrx_to_upper_a("attribution"), "Abution");
    assert_eq!(atxrx_to_upper_a("atrocity"), "Acity");
    assert_eq!(atxrx_to_upper_a("unchanged"), "unchanged");
}

//(104) Write upper-case H to express the word-beginnings hydra- and hydro-
pub fn hydrx_to_upper_h(word: &str) -> String {
    let return_string: String;
    match word {
        w if w.starts_with("hydra") => return_string = format!("H{}", &word[5..]).to_string(),
        w if w.starts_with("hydro") => return_string = format!("H{}", &word[5..]).to_string(),
        _ => return word.to_string(),
    }
    return_string.to_string()
}

#[test]
pub fn test_hydrx_to_upper_h() {
    assert_eq!(hydrx_to_upper_h("hydration"), "Htion");
    assert_eq!(hydrx_to_upper_h("hydrocarbon"), "Hcarbon");
    assert_eq!(hydrx_to_upper_h("unchanged"), "unchanged");
}

// //(60) Write Ag to express aggra-, aggre-, agre-, agri-
// pub fn agxrx_to_upper_ag(word: &str) -> String {
//     let mut return_string: String = word.clone().to_string();
//     match word {
//         w if w.starts_with("aggra") => return_string = format!("Ag{}", &word[5..]).to_string(),
//         w if w.starts_with("aggre") => return_string = format!("Ag{}", &word[5..]).to_string(),
//         w if w.starts_with("agre") => return_string = format!("Ag{}", &word[4..]).to_string(),
//         w if w.starts_with("agri") => return_string = format!("Ag{}", &word[4..]).to_string(),
//         _ => return word.to_string(),
//     }
//     return_string.to_string()
// }

// #[test]
// pub fn test_agxrx_to_upper_ag() {
//     assert_eq!(agxrx_to_upper_ag("aggrevate"), "Agvate");
//     assert_eq!(agxrx_to_upper_ag("aggressive"), "Agssive");
//     assert_eq!(agxrx_to_upper_ag("agriculture"), "Agculture");
//     assert_eq!(agxrx_to_upper_ag("unchanged"), "unchanged");
//     // assert_eq!(agxrx_to_upper_ag("disagree"), "");
// }

fn starts_with_any(word: &str, prefixes: &[&str]) -> bool {
    prefixes.iter().any(|&prefix| word.starts_with(prefix))
}

//(61) Write Dg to express intial degra-, degre-. Very few words use this Principle.
pub fn degrx_to_upper_dg(word: &str) -> String {
    let prefixes = vec!["degra", "degre"];

    if starts_with_any(word, &prefixes) {
        if word.len() > 5 {
            format!("Dg{}", &word[5..])
        } else {
            "Dg".to_string()
        }
    } else {
        word.to_string()
    }
}

#[test]
pub fn test_degrx_to_upper_dg() {
    assert_eq!(degrx_to_upper_dg("degrade"), "Dgde");
    assert_eq!(degrx_to_upper_dg("degree"), "Dge");
}

//
//```
//Agva = aggravate
//Agsv = aggressive
//dsAg = disagree
//Agkl = agriculture
//```
//
//
//```
//Dgdj = degradation
//Dg = degree
//```
//
//(80) Write Ig to express the word-beginnings igne-, igni-, igno-. Very few words use this Principle.
pub fn ignx_to_upper_ig(word: &str) -> String {
    let prefixes = vec!["igne", "igni", "igno"];

    if starts_with_any(word, &prefixes) {
        if word.len() > 4 {
            format!("Ig{}", &word[4..])
        } else {
            "Ig".to_string()
        }
    } else {
        word.to_string()
    }
}

#[test]
pub fn test_ignx_to_upper_ig() {
    assert_eq!(ignx_to_upper_ig("igneous"), "Igous");
    assert_eq!(ignx_to_upper_ig("ignite"), "Igte");
    assert_eq!(ignx_to_upper_ig("ignorance"), "Igrance");
}

//(81) Write Mg to express the word-beginnings magna-, magne-, magni-. The textbook does not mention using this Principle for magno- but the dictionary gives “magnolia” as an example of doing so.
pub fn magnx_to_upper_mg(word: &str) -> String {
    let prefixes = vec!["magna", "magne", "magni", "magno"];

    if starts_with_any(word, &prefixes) {
        if word.len() > 5 {
            format!("Mg{}", &word[5..])
        } else {
            "Mg".to_string()
        }
    } else {
        word.to_string()
    }
}

#[test]
pub fn test_mgnx_to_upper_mg() {
    assert_eq!(magnx_to_upper_mg("magnanimity"), "Mgnimity");
    assert_eq!(magnx_to_upper_mg("magnetize"), "Mgtize");
    assert_eq!(magnx_to_upper_mg("magnitude"), "Mgtude");
    assert_eq!(magnx_to_upper_mg("magnolia"), "Mglia");
}

//(82) Write Sg to express initial signa-, signe-, signi-
pub fn signx_to_upper_sg(word: &str) -> String {
    let prefixes = vec!["signa, signe, signi"];

    if starts_with_any(word, &prefixes) {
        if word.len() > 5 {
            format!("Sg{}", &word[5..])
        } else {
            "Sg".to_string()
        }
    } else {
        word.to_string()
    }
}

#[test]
pub fn test_signx_to_upper_sg() {
    assert_eq!(signx_to_upper_sg("signal"), "Sgl");
    assert_eq!(signx_to_upper_sg("signet"), "Sgt");
    assert_eq!(signx_to_upper_sg("significant"), "Sgficant");
}

//
//Word-beginnings with s- Sounds
//
//(17) Write the letter s to express the prefix some-.
pub fn s_to_some(word: &str) -> String {
    let prefixes = vec!["some"];

    if starts_with_any(word, &prefixes) {
        if word.len() > 4 {
            format!("s{}", &word[4..])
        } else {
            "s".to_string()
        }
    } else {
        word.to_string()
    }
}

#[test]
pub fn test_s_to_some() {
    assert_eq!(s_to_some("something"), "sthing");
    assert_eq!(s_to_some("swhere"), "swhere");
    assert_eq!(s_to_some("unchanged"), "unchanged");
}

// pub fn (word: &str) -> String {
//     let prefixes = vec![""];

//     if starts_with_any(word, &prefixes) {
//         if word.len() > 4 {
//             format!("{}", &word[4..])
//         } else {
//             "".to_string()
//         }
//     } else {
//         word.to_string()
//     }
// }

// #[test]
// pub fn test_() {
//     assert_eq!((""), "");
//     assert_eq!((""), "");
//     assert_eq!((""), "");
// }
//
//```
//sbd, = somebody
//sti = sometime
//```
//
//(77) The letter s represents the word-beginning sub-.
// pub fn (word: &str) -> String {
//     let prefixes = vec![""];

//     if starts_with_any(word, &prefixes) {
//         if word.len() > 4 {
//             format!("{}", &word[4..])
//         } else {
//             "".to_string()
//         }
//     } else {
//         word.to_string()
//     }
// }

// #[test]
// pub fn test_() {
//     assert_eq!((""), "");
//     assert_eq!((""), "");
//     assert_eq!((""), "");
// }
//
//```
//sdu = subdue
//ssq- = subsequent
//sstx = substantial
//```
//
//(59) Write the letter s to express initial and medial ser- (and sur- when it sounds like ser-).
// pub fn (word: &str) -> String {
//     let prefixes = vec![""];

//     if starts_with_any(word, &prefixes) {
//         if word.len() > 4 {
//             format!("{}", &word[4..])
//         } else {
//             "".to_string()
//         }
//     } else {
//         word.to_string()
//     }
// }

// #[test]
// pub fn test_() {
//     assert_eq!((""), "");
//     assert_eq!((""), "");
//     assert_eq!((""), "");
// }
//
//```
//ksv = conserve
//sva = survey
//sp- = serpent
//```
//
//(92) Write C to express the word-beginnings circ-, circu-, circum-.
//
//```
//Cl = circle
//Cla = circulate
//Cv- = circumvent
//```
//
//Word-beginnings with k-, Hard c-, or qu- Sounds
//
//(34) Write the letter k to express initial or medial cog-, col-, com-, con-, cor-, coun-, cum-.
//
//```
//knj = cognition        knz = colonize
//kf/ = comfort          ks/ = concert
//ksp- = correspond      akt = account
//nkb- = incumbent
//```
//
//(39) Write an upper-case K to express the contin-, contra-, contri-, contro-, counter-.
//
//```
//Ku = continue          Kdk = contradict
//Kv = contrive          Kl = control
//nK = encounter
//```
//
//(54) Write the letter q to express qu-.
//
//```
//qn = queen
//sq- = squint
//```
//
//(55) Write upper-case Q to express quadr-.
//
//```
//Q- = quadrant
//Qpd = quadruped
//```
//
//Other Word-beginnings
//
//(41) Write the letter p to express initial or medial per-, pur-, pr-, pre-, pro-.
//
//```
//pfk = perfect          pcs = purchase
//spg = spring           p' = press
//pdk = predict          pb = probe
//```
//
//(57) Write upper-case L to express initial sounds like letter-, litter-, liter-.
//
//```
//Lhd = letterhead       Lbx = litterbox
//Ll = literal           L; = literate
//```
//
//(58) Upper-case N represents initial or medial enter-, inter-, intri-, intro-, intel-.
//
//```
//Ntn = entertain        N, = interest
//Nka = intricate        Ndkj = introduction
//Nk = intellect         uNjb = unintelligible
//```
//
//(78) Write El to express electri-, electro-.
//
//```
//El; = electricity      Elf = electrify
//Elj = electrician      ElMgI = electro-magnetism
//Elnk = electronic      Eld = electrode
//```
//
//(90) Write upper-case P to express initial or medial para-, peri-, pira-. (“Pyra-” and “pyro-” are not mentioned in the textbook but the dictionary does give some examples of P representing those word-beginnings.)
//
//```
//kPv = comparative      aP- = apparent
//Pd = period            PMe = perimeter
//Pwql = periwinkle      Ps, = piracy
//Pmd = pyramid          Ptkk = pyrotechnic
//```
//
//(95) Write a- to express the word-beginnings anta-, ante-, anti-.
//
//```
//a-spk = antiseptic     a-spa = anticipate
//a-dlv = antediluvian   a-pnl = antepenultimate
//a-gnz = antagonize
//```
//
//(105) Write Ml to express the word-beginning multi-.
//

//Mltu = multitude       Mlp = multiple

//## Word-beginnings
//
//Vowels Followed by n, m, r
//
//(40) Write the letter n to express initial en-, in- [Implemented]
//
//```d
//nlj = enlarge // works
//ndw = endow // works
//nfm = inform //works
//ns, = insist //works
//```
//
//(46) Write the letter u to express “un” at the beginning of a word. [Implemented]
//
//```
//uwz = unwise // works
//ukdj = unconditional // works
//```
//
//(75) Write i to express initial im-. This Principle can also be used when there are prefixes added to word that started out with im-, but this Principle does not apply to words like “crimson” or “simmer” that just happen to contatin the sequence I+M. [Partially Implemented]
//
//```
//ij = image // works
//ita = imitate // works
//uipa = unimpaired //unimplemented
//uipv = unimproved //unimplemented
//```
//
//(89) Write m to express initial and medial em or um
//
//```
//mfsz = emphasize // works
//mt, = empty // works
//mpi = umpire // works
//mbK = umbilical // works
//```
//
//(42) Write a to express initial ar-, e for er-, o for or-, u for ur-. Apparently this Principle only applies to words in which the vowel and the r are members of the same syllable. For exmaple, you cannot use this Principle to write “arise” or “erase.”
//
//```
//ac = arch // works
//eth = earth // works
//ojn = origin //wo
//ubn = urban // works
//```
//
//Vowels Dropped from Selected Prefixes
//
//(56) Omit the vowels in be-, de-, di-, dis-, mis-, re-. Although it is not mentioned in the textbook, the dictionary indicates that the vowel is also omitted in bi-.
//
//```
//bsi = beside // works          blad = belated // works
//bpd = biped // works           bsK = bicycle // works
//dtk = detect // works          dtc- = detachment // works
//drk = direct // works          drnl = diurnal // works
//dspl = dispel // works         dsnfk = disinfect // works
//msjj = misjudge // works       msUs- = misunderstand // works
//rmv = remove // works          rdkj = reduction // works
//```
//
//Word-beginnings with -nkl- and -kl- Sounds
//
//(69) Write nc to express the word-beginnings encli-, enclo-, incle-, incli-, inclo-, inclu-.
//
//```
//ncT = enclitic // works         ncnj = inclination // works
//ncz/ = enclosure // works       nc = inclose // works
//nc- = inclement // works        ncd = include // works
//```

pub fn test_module_file() {
    println!("words/beginnings.rs connected");
}
