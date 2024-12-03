use std::collections::HashMap;
use reformation::{Reformation, Regex};
use sorted_vec::SortedVec;

struct NumberPair {
    first: Vec<i128>,
    second: Vec<i128>,
}

impl NumberPair {
    fn joinWithAnother(&self, other: &NumberPair) -> NumberPair {
        NumberPair {
            first: self.first.iter().chain(other.first.iter()).cloned().collect(),
            second: self.second.iter().chain(other.second.iter()).cloned().collect(),
        }
    }
}

pub fn day1() {
    let input = include_str!("../input/day1.txt");
    let re = Regex::new(r"(\d)\s(\d)").unwrap();
    let mut numbers = input
        .lines()
        .map(|line| {
            println!("{}", line);
            let caputredLine = re.captures(line).unwrap();
            NumberPair {
                first: vec![caputredLine.get(1).unwrap().as_str().parse::<i128>().unwrap()],
                second: vec![caputredLine.get(2).unwrap().as_str().parse::<i128>().unwrap()],
            }
        })
        .collect::<Vec<NumberPair>>();
    let mut allNumbers = numbers.pop().unwrap();
    let mut finalPair = numbers.iter().fold(allNumbers, |acc, number| acc.joinWithAnother(number));

    finalPair.first.sort_by(|a, b| b.cmp(a));
    finalPair.second.sort_by(|a, b| b.cmp(a));

    let mut sums = Vec::new();
    for first in finalPair.first.iter() {
        for second in finalPair.second.iter() {
            sums.push(first - second);
        }
    }

    let sum: i128 = sums.iter().sum();
    println!("{}", sum);
}