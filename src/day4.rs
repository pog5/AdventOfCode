fn part1(input: &str) -> String {
    let oneline_captures: i128 = input
        .as_bytes()
        .windows(4)
        .filter(|&w| w == b"XMAS" || w == b"SAMX")
        .count() as i128;

    eprintln!("one-line caps: {}", oneline_captures);

    let mut multiline_captures: i128 = 0;
    let _: Vec<_> = input
        .lines()
        .map_windows(|&[a, b, c, d]| {
            eprintln!();
            eprintln!("a: {}", a);
            eprintln!("b: {}", b);
            eprintln!("c: {}", c);
            eprintln!("d: {}", d);
            let mut i = 0;
            while i < a.len() {
                eprintln!("i: {}", i);
                let singlerow = format!(
                    "{}{}{}{}",
                    a.chars().nth(i).unwrap(),
                    b.chars().nth(i).unwrap(),
                    c.chars().nth(i).unwrap(),
                    d.chars().nth(i).unwrap()
                );
                if singlerow == "XMAS" {
                    multiline_captures += 1;
                    eprintln!("singlerow caught");
                }
                if singlerow == "SAMX" {
                    multiline_captures += 1;
                    eprintln!("singlerow_rev caught");
                }

                if i > 2 {
                    let multirow_backwards = format!(
                        "{}{}{}{}",
                        a.chars().nth(i).unwrap(),
                        b.chars().nth(i - 1).unwrap(),
                        c.chars().nth(i - 2).unwrap(),
                        d.chars().nth(i - 3).unwrap()
                    );
                    if multirow_backwards == "XMAS" {
                        multiline_captures += 1;
                        eprintln!("multirow_backwards caught");
                    }
                    if multirow_backwards == "SAMX" {
                        multiline_captures += 1;
                        eprintln!("multirow_backwards_rev caught");
                    }
                }

                if i < a.len() - 3 {
                    let multirow = format!(
                        "{}{}{}{}",
                        a.chars().nth(i).unwrap(),
                        b.chars().nth(i + 1).unwrap(),
                        c.chars().nth(i + 2).unwrap(),
                        d.chars().nth(i + 3).unwrap()
                    );

                    if multirow == "XMAS" {
                        multiline_captures += 1;
                        eprintln!("multirow caught");
                    }
                    if multirow == "SAMX" {
                        multiline_captures += 1;
                        eprintln!("multirow_rev caught");
                    }
                }
                i += 1;
            }
            true
        })
        .collect();

    eprintln!("multi-line caps: {}", multiline_captures);

    let sum = oneline_captures + multiline_captures;
    eprintln!("total: {}", sum);

    sum.to_string()
}

fn part2(input: &str) -> String {
    let mut multiline_captures: i128 = 0;
    let _: Vec<_> = input
        .lines()
        .map_windows(|&[a, b, c]| {
            eprintln!();
            eprintln!("a: {}", a);
            eprintln!("b: {}", b);
            eprintln!("c: {}", c);
            let mut i = 0;
            while i < a.len() {
                eprintln!("i: {}", i);

                if i < a.len() - 2 {
                    let multirow = format!(
                        "{}{}{}",
                        a.chars().nth(i).unwrap(),
                        b.chars().nth(i + 1).unwrap(),
                        c.chars().nth(i + 2).unwrap(),
                    );

                    if multirow == "MAS" {
                        // eprintln!("normal caught");
                    }
                    if multirow == "SAM" {
                        // eprintln!("backwards caught");
                    }

                    let first = multirow == "MAS" || multirow == "SAM";

                    let multirow_backwards = format!(
                        "{}{}{}",
                        a.chars().nth(i + 2).unwrap(),
                        b.chars().nth(i + 1).unwrap(),
                        c.chars().nth(i).unwrap(),
                    );
                    if multirow_backwards == "MAS" {
                        // eprintln!("flipped caught");
                    }
                    if multirow_backwards == "SAM" {
                        // eprintln!("flipped backwards caught");
                    }

                    let second = multirow_backwards == "MAS" || multirow_backwards == "SAM";

                    if first && second {
                        multiline_captures += 1;
                        // eprintln!("both");
                    }
                }

                i += 1;
            }
            true
        })
        .collect();

    let sum = multiline_captures;
    eprintln!("total: {}", sum);

    sum.to_string()
}

pub fn day4() {
    let input = include_str!("../input/day4.txt");
    let response = part1(input);
    println!();
    let response2 = part2(input);

    println!();
    println!("Part One: {}", response);
    println!("Part Two: {}", response2);
}
