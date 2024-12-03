use std::collections::HashMap;

fn is_safe(line: &str) -> bool {
    let mut nums = Vec::new();
    for num in line.split(" ") {
        let parsed: i128 = num.parse().unwrap();
        nums.push(parsed);
    };

    if !(nums.windows(2).all(|w| w[0] < w[1]) || nums.windows(2).all(|w| w[0] > w[1])) {
        return false;
    }

    nums.windows(2).all(|w| {
        let diff = if w[0] > w[1] {
            w[0] - w[1]
        } else {
            w[1] - w[0]
        };

        diff >= 1 && diff <= 3
    })
}

fn part1(input: &str) -> String {
    let mut map = HashMap::new();
    for line in input.lines() {
        let issafe = is_safe(line);
        println!("{:?}, {}", line, issafe);
        map.insert(line, issafe);
    }

    let safe: i128 = map.iter().filter(|pair| *pair.1).count() as i128;

    let solution = format!("{}", safe);

    solution
}

pub fn day2() {
    let input = include_str!("../input/day2.txt");
    let response = part1(input);

    println!();
    println!("Part One: {}", response);
}