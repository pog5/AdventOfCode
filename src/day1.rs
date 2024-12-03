use std::collections::HashMap;
use reformation::Regex;

fn dist_between_nums(a: i128, b: i128) -> i128 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn part1(input: &str) -> String {
    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    let mut first_list = Vec::new();
    let mut second_list = Vec::new();

    for line in input.lines() {
        let captured = re.captures(line).expect("Can't capture from line");
        first_list.push(captured.get(1).unwrap().as_str().parse::<i128>().unwrap());
        second_list.push(captured.get(2).unwrap().as_str().parse::<i128>().unwrap());
    }

    first_list.sort();
    second_list.sort();

    // println!("{:?}", first_list);
    // println!("{:?}", second_list);

    let mut nums = first_list.iter().zip(second_list.iter())
        .map(|(a, b)| dist_between_nums(*a, *b))
        .collect::<Vec<i128>>();

    nums.sort();
    // println!("{:?}", nums);

    let sum: i128 = nums.iter().sum();
    println!("{}", sum);
    return sum.to_string();
}

fn part2(input: &str) -> String {
    let mut map: HashMap<i128, i128> = HashMap::new();
    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    let mut first_list = Vec::new();
    let mut second_list = Vec::new();

    for line in input.lines() {
        let captured = re.captures(line).expect("Can't capture from line");
        first_list.push(captured.get(1).unwrap().as_str().parse::<i128>().unwrap());
        second_list.push(captured.get(2).unwrap().as_str().parse::<i128>().unwrap());
    }

    for first in first_list {
        for second in &*second_list {
            if (first != *second)  {
                continue;
            }

            if !map.contains_key(&first) {
                map.insert(first, 1);
            } else {
                map.insert(first, map.get(&first).unwrap() + 1);
            }

        }
    }

    let mut alltogether = Vec::new();
    for pair in map {
        alltogether.push(pair.0 * pair.1);
    }

    let result: i128 = alltogether.iter().sum();
    println!("{}", result);
    return result.to_string();
}

pub fn day1() {
    let input = include_str!("../input/day1.txt");
    let response = part1(input);
    let response2 = part2(input);

    println!();
    println!("Part One: {}", response);
    println!("Part Two: {}", response2);
}