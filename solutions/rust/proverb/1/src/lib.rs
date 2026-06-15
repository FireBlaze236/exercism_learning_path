pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();
    if list.is_empty() {
        return String::new();
    }

    for pair in list.windows(2) {
        match pair {
            [first, second] => {
                proverb = proverb + &format!("For want of a {} the {} was lost.\n", first, second)
            }
            _ => {}
        }
    }
    if list.len() >= 1 {
        proverb = proverb + &format!("And all for the want of a {}.", list[0]);
    }

    return proverb;
}
