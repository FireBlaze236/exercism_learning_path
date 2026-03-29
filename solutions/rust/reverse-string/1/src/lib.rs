pub fn reverse(input: &str) -> String {
    let mut rev_string = String::new();

    for c in input.chars().rev() {
        rev_string.push(c);
    }

    rev_string
}
