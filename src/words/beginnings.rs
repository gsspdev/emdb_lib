pub fn test_module_file() {
    println!("words/beginnings.rs connected");
}

pub fn init_un_to_u(word: &str) -> String {
    match word {
        w if w.starts_with("un") => word.replacen("un", "u", 1),
        _ => word.to_string(),
    }
}

pub fn init_en_in_to_n(word: &str) -> String {
    match word {
        w if w.starts_with("en") => word.replacen("en", "n", 1),
        w if w.starts_with("in") => word.replacen("in", "n", 1),
        _ => word.to_string(),
    }
}

pub fn init_im_to_i(word: &str) -> String {
    match word {
        w if w.starts_with("im") &&
            w.chars().nth(2).map_or(false, |c| c == 'a' || c == 'i') => {
            let without_im = w.replacen("im", "i", 1);
            format!("{}{}", &without_im[0..1], &without_im[2..])
        },
        w if w.starts_with("im") => {
            w.replacen("im", "i", 1)
        },
        _ => word.to_string(),
    }
}

pub fn em_um_to_m(word: &str) -> String {
    match word {
        w if !w.ends_with("em") && w.contains("em") => w.replace("em", "m"),
        w if !w.ends_with("um") && w.contains("um") => w.replace("um", "m"),
        _ => word.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_init_un_to_u() {
        assert_eq!(init_un_to_u("tester"), "tester");
        assert_eq!(init_un_to_u("unwise"), "uwise".to_string());
    }

    #[test]
    fn test_init_en_in_to_n() {
        assert_eq!(init_en_in_to_n("unmodified"), "unmodified");
        assert_eq!(init_en_in_to_n("enlarge"), "nlarge");
        assert_eq!(init_en_in_to_n("endow"), "ndow");
        assert_eq!(init_en_in_to_n("inform"), "nform");
        assert_eq!(init_en_in_to_n("insist"), "nsist");
    }
    
    #[test]
    fn test_init_im_to_n() {
        assert_eq!(init_en_in_to_n("unmodified"), "unmodified");
        assert_eq!(init_im_to_i("image"), "ige");
        assert_eq!(init_im_to_i("imitate"), "itate");
        assert_eq!(init_im_to_i("imply"), "iply");
//        assert_eq!(init_im_to_in("unimpaired"), "unpaired".to_string());
//        assert_eq!(init_im_to_in("unimproved"), "unproved".to_string());
    }

    #[test]
    fn test_em_um_to_m() {
        assert_eq!(em_um_to_m("unmodified"), "unmodified");
        assert_eq!(em_um_to_m("emphasize"), "mphasize");
        assert_eq!(em_um_to_m("empty"), "mpty");
        assert_eq!(em_um_to_m("umpire"), "mpire");
        assert_eq!(em_um_to_m("umbilical"), "mbilical");
}

}

//## Word-beginnings
//
//Vowels Followed by n, m, r
//
//(40) Write the letter n to express initial en-, in- [Implemented]
//
//```
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
//ac = arch
//eth = earth
//ojn = origin
//ubn = urban
//```
//
//Vowels Dropped from Selected Prefixes
//
//(56) Omit the vowels in be-, de-, di-, dis-, mis-, re-. Although it is not mentioned in the textbook, the dictionary indicates that the vowel is also omitted in bi-.
//
//```
//bsi = beside           blad = belated
//bpd = biped            bsK = bicycle
//dtk = detect           dtc- = detachment
//drk = direct           drnl = diurnal
//dspl = dispel          dsnfk = disinfect
//msjj = misjudge        msUs- = misunderstand
//rmv = remove           rdkj = reduction
//```
//
//Word-beginnings with -nkl- and -kl- Sounds
//
//(69) Write nc to express the word-beginnings encli-, enclo-, incle-, incli-, inclo-, inclu-.
//
//```
//ncT = enclitic         ncnj = inclination
//ncz/ = enclosure       nc = inclose
//nc- = inclement        ncd = include
//```
//
//(70) Write dc to express the word-beginnings decla-, decle-, decli-.
//
//```
//dcv = declarative
//dcj = declension
//dcn = decline
//```
//
//(71) Write rc to express the word-beginnings recla-, recli-, reclu-.
//
//```
//rcm = reclaim
//rcn = recline
//rcsv = reclusive
//```
//
//Word-beginnings with -tr- and -dr- Sounds
//
//(63) Use upper-case D to express deter- or detri-
//
//```
//Dm = determine
//D-l = detrimental
//```
//
//(64) Write Al to express initial or medial alter-
//
//```
//Alnv = alternative
//Alj = alteration
//```
//
//(66) Pa represents initial pater-, patri-, patro-
//
//```
//Pan; = paternity
//Pat = patriot
//Pan = patron
//```
//
//(85) Write upper-case T to express the prefix trans-
//
//```
//Tf = transfer
//Tmub = transmutable
//```
//
//(88) Write upper-case X to express the word-beginnings exter-, extir-, extra-, extre-, extri-, extru-.
//
//```
//Xnl = external
//Xk = extract
//Xm = extreme
//Xka = extricate
//Xd = extrude
//```
//
//(96) Write upper-case R to express the word-beginnings retre-, retri-, retro-
//
//```
//Rt = retreat
//Rbj = retribution
//Rspkv = retrospective
//```
//
//(102) Write upper-case A to express the word-beginnings attra-, attri-, atro-
//
//```
//Akv = attractive
//Abj = attribution
//As; = atrocity
//```
//
//(104) Write upper-case H to express the word-beginnings hydra- and hydro-
//
//```
//Hj = hydration
//Hkbn = hydrocarbon
//```
//
//Word-beginnings with -gr- and -gn- Sounds
//
//(60) Write Ag to express aggra-, aggre-, agre-, agri-
//
//```
//Agva = aggravate
//Agsv = aggressive
//dsAg = disagree
//Agkl = agriculture
//```
//
//(61) Write Dg to express intial degra-, degre-. Very few words use this Principle.
//
//```
//Dgdj = degradation
//Dg = degree
//```
//
//(80) Write Ig to express the word-beginnings igne-, igni-, igno-. Very few words use this Principle.
//
//```
//Igx = igneous
//Igt = ignite
//Igc/ = ignorance
//```
//
//(81) Write Mg to express the word-beginnings magna-, magne-, magni-. The textbook does not mention using this Principle for magno- but the dictionary gives “magnolia” as an example of doing so.
//
//```
//Mgn; = magnanimity
//Mgtz = magnetize
//Mgtu = magnitude
//Mgla = magnolia
//```
//
//(82) Write Sg to express initial signa-, signe-, signi-
//
//```
//Sgl = signal
//Sgt = signet
//Sgfk- = significant
//```
//
//Word-beginnings with s- Sounds
//
//(17) Write the letter s to express the prefix some-.
//
//```
//sbd, = somebody
//sti = sometime
//```
//
//(77) The letter s represents the word-beginning sub-.
//
//```
//sdu = subdue
//ssq- = subsequent
//sstx = substantial
//```
//
//(59) Write the letter s to express initial and medial ser- (and sur- when it sounds like ser-).
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
