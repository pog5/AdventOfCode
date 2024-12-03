use reformation::{Reformation, Regex};

fn dist_between_nums(a: i128, b: i128) -> i128 {
    if a > b {
        a - b
    } else {
        b - a
    }
}
pub fn day1() {
    let input = include_str!("../input/day1.txt");
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

    println!("{:?}", first_list);
    println!("{:?}", second_list);

    let mut nums = first_list.iter().zip(second_list.iter())
        .map(|(a, b)| dist_between_nums(*a, *b))
        .collect::<Vec<i128>>();

    nums.sort();
    println!("{:?}", nums);

    let sum: i128 = nums.iter().sum();
    println!("{}", sum);
}