pub fn str_pair_lookup<'a>(it: &'a [(&str, &str)], key: &str) -> Option<&'a str> {
    it.iter()
        .find_map(|(x, y)| if *x == key { Some(*y) } else { None })
}
pub fn string_pair_lookup<'a>(it: &'a [(String, String)], key: &str) -> Option<&'a String> {
    it.iter()
        .find_map(|(x, y)| if *x == key { Some(y) } else { None })
}
