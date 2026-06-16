use std::{collections::HashMap, iter::Map};

pub fn find_muls(limit: u32, n: u32) -> Vec<u32> {
    let mut t: Vec<u32> = Vec::new();

    if n == 0 {
        t.push(0);
        return t;
    }

    let max = limit / n;

    for i in 1..=max {
        t.push(i * n);
    }

    t
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    //todo!("Sum the multiples of all of {factors:?} which are less than {limit}")

    let mut v: Vec<u32> = Vec::new();
    for i in factors {
        let m = find_muls(limit, *i);
        v.extend_from_slice(&m);

        for x in &v {
            println!("{}: {}", i, x);
        }
    }

    let mut sum = 0;
    let mut seen_map: HashMap<u32, bool> = HashMap::new();
    for x in v {
        if seen_map.contains_key(&x) || x == limit {
            continue;
        }
        sum += x;
        seen_map.insert(x, true);
    }

    return sum;
}
