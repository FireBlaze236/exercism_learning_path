pub fn annotate(garden: &[&str]) -> Vec<String> {
    let mut result = Vec::new();

    for y in 0..garden.len() {
        let mut rrow = String::new();
        for (idx, ch) in garden[y].char_indices() {
            if ch == '*' {
                rrow.push(ch);
            } else {
                let dirs = [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 0),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ];

                let mut ncount = 0;
                for (dr, dc) in dirs {
                    let rr = y as isize + dr;
                    let cc = idx as isize + dc;

                    if rr >= 0 && rr < garden.len() as isize {
                        let nrow: Vec<char> = garden[rr as usize].chars().collect();
                        if cc >= 0 && cc < nrow.len() as isize {
                            let n = nrow[cc as usize];
                            if n == '*' {
                                ncount += 1;
                            }
                        }
                    }
                }

                let c = match ncount {
                    0 => ' ',
                    1 => '1',
                    2 => '2',
                    3 => '3',
                    4 => '4',
                    5 => '5',
                    6 => '6',
                    7 => '7',
                    8 => '8',
                    _ => ' ',
                };

                rrow.push(c);
            }
        }

        result.push(rrow);
    }

    println!("{:?}", garden);

    result
}
