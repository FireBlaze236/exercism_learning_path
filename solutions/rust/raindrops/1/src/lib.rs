pub fn raindrops(n: u32) -> String {
    let mut res = String::new();
    let mut no_div = true;
    if n.is_multiple_of(3) {
        res.push_str("Pling");
        no_div = false;
    }
    if n.is_multiple_of(5) {
        res.push_str("Plang");
        no_div = false;
    }
    if n.is_multiple_of(7) {
        res.push_str("Plong");
        no_div = false;
    }

    if no_div {
        res += &n.to_string();
    }

    res
}
