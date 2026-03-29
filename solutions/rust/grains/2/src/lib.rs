pub fn square(s: u32) -> u64 {
    1 << (s - 1)
}

pub fn total() -> u64 {
    (1u32..=64u32).map(square).sum()
}
