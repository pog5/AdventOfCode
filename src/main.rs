use std::io::Write;
use crate::day1::day1;
use crate::day2::day2;
use crate::day3::day3;

mod day1;
mod day2;
mod day3;

fn main() {
    print!("Day: ");
    std::io::stdout().flush().expect("cant flush stdout");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let day = input.trim().parse::<u32>().unwrap();

    match day {
        1 => day1(),
        2 => day2(),
        3 => day3(),
        _ => println!("Day not implemented"),
    }
}

