pub fn egg_count(display_value: u32) -> usize {
    let mut t = display_value;
    let mut count = 0;
    while t != 0 {
        count += t & 0x1;
        t >>= 1;
    }
    count as usize
}
