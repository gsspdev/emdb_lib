pub fn test_module_file() {
    println!("words/endings_freq.rs connected");
}

pub fn final_ly_ily_to_l(word: &str) -> String {
    if word.ends_with("ily") {
        return format!("{}l", &word[..word.len() - 3]);
    }
    else if word.ends_with("ly") {
        return format!("{}l", &word[..word.len() - 2]);
    }
    word.to_string()
}

pub fn final_ing_thing_to_g(word: &str) -> String {
    if word.ends_with("thing") {
        return format!("{}g", &word[..word.len() - 5]);
    }
    else if word.ends_with("ing") {
        return format!("{}g", &word[..word.len() - 3]);
    }
    word.to_string()
}

pub fn final_ary_ery_iry_ory_ury_to_y(word: &str) -> String {
    if word.ends_with("orry") { return format!("{}y", &word[..word.len() - 4]); }
    else if word.ends_with("airy") { return format!("{}y", &word[..word.len() - 4]); }
    else if word.ends_with("ary") { return format!("{}y", &word[..word.len() - 3]); }
    else if word.ends_with("iry") { return format!("{}y", &word[..word.len() - 3]); }
    else if word.ends_with("ery") { return format!("{}y", &word[..word.len() - 3]); }
    else if word.ends_with("ory") { return format!("{}y", &word[..word.len() - 3]); }
    else if word.ends_with("ury") { return format!("{}y", &word[..word.len() - 3]); }
    else if word.ends_with("ari") { return format!("{}y", &word[..word.len() - 3]); }
    else if word.ends_with("ry") { return format!("{}y", &word[..word.len() - 2]); }
    word.to_string()
}

#[cfg(test)]
    mod tests {
    use super::*;

    #[test]
    fn test_final_ly_ily_to_l() {
        assert_eq!(final_ly_ily_to_l("happily"), "happl".to_string());
        assert_eq!(final_ly_ily_to_l("greatly"), "greatl".to_string());
    }

    #[test]
    fn test_final_ing_thing_to_g() {
        assert_eq!(final_ing_thing_to_g("anything"), "anyg".to_string());
        assert_eq!(final_ing_thing_to_g("running"), "runng".to_string());
    }

    #[test]
    fn test_final_ary_ery_iry_ory_ury_to_y() {
        assert_eq!(final_ary_ery_iry_ory_ury_to_y("fairy"), "fy".to_string());
        assert_eq!(final_ary_ery_iry_ory_ury_to_y("flattery"), "flatty".to_string());
        assert_eq!(final_ary_ery_iry_ory_ury_to_y("sorry"), "sy".to_string());
        assert_eq!(final_ary_ery_iry_ory_ury_to_y("wiry"), "wy".to_string());
        assert_eq!(final_ary_ery_iry_ory_ury_to_y("surgery"), "surgy".to_string());
        assert_eq!(final_ary_ery_iry_ory_ury_to_y("hurry"), "hury".to_string());
//        assert_eq!(final_ary_ery_iry_ory_ury_to_y("glorious"), "glyous".to_string()); // this test will fail
//        assert_eq!(final_ary_ery_iry_ory_ury_to_y("nefarious"), "nefyous".to_string()); // this test will fail
    }
}
