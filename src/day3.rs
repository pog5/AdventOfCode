use reformation::Regex;

fn part1(input: &str) -> String {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut numbers = Vec::new();
    for (_, [first, second]) in re.captures_iter(input).map(|c| c.extract()) {
        let firstnum: i128 = first.parse::<i128>().unwrap();
        let secondnum: i128 = second.parse::<i128>().unwrap();
        numbers.push(firstnum * secondnum);
    }
    
    let sum: i128 = numbers.iter().sum();
    sum.to_string()
}

fn part2(input: &str) -> String {
    "TODO".to_string()
}

pub fn day3() {
    let input = include_str!("../input/day3.txt");
    let response = part1(input);
    println!();
    let response2 = part2(input);

    println!();
    println!("Part One: {}", response);
    println!("Part Two: {}", response2);
}