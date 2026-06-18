pub fn get_opening(c: char) -> Option<char> {
    let r = Some(match &c {
        ')' => '(',
        '}' => '{',
        ']' => '[',
        _ => ' ',
    });

    return r;
}
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut s = Vec::new();

    let openings = vec!['(', '{', '['];
    let closings = vec![')', '}', ']'];

    for c in string.chars() {
        if openings.contains(&c) {
            s.push(c);
        } else if closings.contains(&c) {
            if s.last() == get_opening(c).as_ref() {
                s.pop();
            } else {
                return false;
            }
        }
    }

    return s.is_empty();
}
