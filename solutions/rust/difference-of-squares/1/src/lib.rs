pub fn square_of_sum(n: u32) -> u32 {
    let range = 1..=n;
    let sum: u32 = range.sum();
    sum * sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    let range = 1..=n;
    range.map(|v| v * v).sum()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
