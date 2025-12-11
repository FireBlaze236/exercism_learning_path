/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // todo!("Is the Luhn checksum for {code} valid?");

    if code.len() <= 1 {
        return false;
    }

    let mut digits = Vec::new();
    for c in code.chars() {
        let n = c.to_digit(10);
        if let Some(x) = n {
            digits.push(x);
        }
    }

    if code.chars().any(|c| !c.is_digit(10) && !c.is_whitespace()) {
        return false;
    }

    digits.reverse();

    if digits.len() <= 1 {
        return false;
    }

    let mut double = false;
    for didx in 0..digits.len() {
        if double {
            let mut nx = digits[didx] * 2;
            if nx > 9 {
                nx -= 9;
            }
            digits[didx] = nx;
        }
        double = !double;
    }

    let sum: u32 = digits.iter().sum();
    println!("{:?}", digits);
    println!("{}", sum);

    sum.is_multiple_of(10)
}
