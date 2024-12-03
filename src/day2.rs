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

fn is_safe_p2(line: &str, is_recursing: bool) -> bool {
    let mut nums = Vec::new();
    for num in line.split(" ") {
        let parsed: i128 = num.parse().unwrap();
        nums.push(parsed);
    };

    let mut failedOrder = false;
    if !(nums.windows(2).all(|w| w[0] < w[1]) || nums.windows(2).all(|w| w[0] > w[1])) {
        failedOrder = true;
    }

    let normal = nums.windows(2).all(|w| {
        let diff = if w[0] > w[1] {
            w[0] - w[1]
        } else {
            w[1] - w[0]
        };

        diff >= 1 && diff <= 3
    });

    if (normal && !failedOrder) {
        true
    } else {
        if (!is_recursing) {
            let mut ournums = Vec::new();
            for num in line.split(" ") {
                let parsed: i128 = num.parse().unwrap();
                ournums.push(parsed);
            };
            let mut i = 0;
            while i < ournums.len() {
                let mut copied = ournums.clone();
                let removed = copied.remove(i);
                let mut stringed: Vec<String> = Vec::new();
                copied.iter().for_each(|num| stringed.push(num.to_string()));

                let problem_solved = is_safe_p2(&*stringed.join(" "), true);
                if problem_solved {
                    eprintln!("solved by removing {} from {:?}", removed, copied);
                    return true;
                }
                i = i + 1;
            }
            false
        } else {
            false
        }
    }
}

fn part2(input: &str) -> String {
    let mut map = HashMap::new();
    for line in input.lines() {
        let issafe = is_safe_p2(line, false);
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
    let response2 = part2(input);

    println!();
    println!("Part One: {}", response);
    println!("Part Two: {}", response2);
}