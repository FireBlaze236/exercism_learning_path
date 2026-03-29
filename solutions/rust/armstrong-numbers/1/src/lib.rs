pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits: Vec<u32> = Vec::new();

    let mut temp = num;
    while temp != 0 {
        let d = temp % 10;
        temp /= 10;
        digits.push(d);
    }

    println!("{:?}", digits);

    let l: u32 = digits.len() as u32;
    let sum: u32 = digits.iter().map(|&d| d.pow(l)).sum();
    println!("{:?}", l);
    println!("{:?}", sum);
    sum == num
}
