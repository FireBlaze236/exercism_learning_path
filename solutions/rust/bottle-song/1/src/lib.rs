use std::mem::take;

pub fn get_bottle_name(bottle_number: u32) -> String {
    let st = match bottle_number {
        0 => "No",
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        10 => "Ten",
        _ => "Unknown",
    };

    st.to_string()
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut res = String::new();
    let mut bottles = start_bottles;
    for i in 1..=take_down {
        let before_bottles = get_bottle_name(bottles);
        let s1 = if bottles == 1 { "" } else { "s" };
        res += format!("{} green bottle{s1} hanging on the wall,\n", before_bottles).as_str();
        res += format!("{} green bottle{s1} hanging on the wall,\n", before_bottles).as_str();
        bottles -= 1;
        let after_bottles = get_bottle_name(bottles);
        res += format!("And if one green bottle should accidentally fall,\n").as_str();
        let s = if bottles == 1 { "" } else { "s" };
        res += format!(
            "There'll be {} green bottle{s} hanging on the wall.\n",
            after_bottles.to_lowercase()
        )
        .as_str();

        if i != take_down {
            res += "\n";
        }
    }

    res
}
