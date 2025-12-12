use std::{collections::HashMap, u32};

pub fn nth(n: u32) -> u32 {
    let mut num: u32 = 3;
    let mut sieve: HashMap<u32, bool> = HashMap::new();
    let max: u32 = 200_000u32;
    for i in 2..max {
        if i != 2 && i.is_multiple_of(2u32) {
            sieve.insert(i, false);
            continue;
        }
        sieve.insert(i, true);
    }
    while num <= max {
        if sieve.get(&num) == Some(&true) {
            let mut t = num + num;
            while t <= max {
                sieve.insert(t, false);
                t += num;
            }
        }
        num += 2;
    }

    let mut primes: Vec<u32> = sieve.iter().filter(|e| *e.1).map(|e| *e.0).collect();
    primes.sort();

    // for i in primes {
    //     println!("{}", i);
    // }

    return primes[n as usize];
}
