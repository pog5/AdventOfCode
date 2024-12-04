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

fn find_vector_with_closest_backwards_value<'a>(vec1: &'a Vec<i128>, vec2: &'a Vec<i128>, target: &i128) -> &'a Vec<i128> {
    // Filter vec1 to values less than or equal to target and find max
    let max_backward1 = vec1.iter()
        .filter(|&&x| x <= *target)
        .max()
        .cloned();

    // Filter vec2 to values less than or equal to target and find max
    let max_backward2 = vec2.iter()
        .filter(|&&x| x <= *target)
        .max()
        .cloned();

    // Compare the maximum backward values
    match (max_backward1, max_backward2) {
        (Some(val1), Some(val2)) => {
            if val1 >= val2 {
                vec1
            } else {
                vec2
            }
        },
        (Some(_), None) => vec1,
        (None, Some(_)) => vec2,
        (None, None) => vec1  // Default to first vector if no values found
    }
}

fn part2(input: &str) -> String {
    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();
    eprintln!("line: {:?}", input);

    let mut do_locations: Vec<i128> = do_re.find_iter(input).map(|m| m.end() as i128).collect();
    do_locations.splice(0..0, vec![0]);
    
    let mut dont_locations: Vec<i128> = dont_re.find_iter(input).map(|m| m.end() as i128).collect();
    do_locations.sort();
    dont_locations.sort();
    println!("do: {:?}", do_locations);
    println!("dont: {:?}", dont_locations);

    let mut numbers = Vec::new();

    for mul_match in mul_re.find_iter(input) {
        let mul_location = mul_match.start() as i128;

        let allowed_to_mul = *find_vector_with_closest_backwards_value(&do_locations, &dont_locations, &mul_location) == do_locations;

        eprintln!("match: {:?}, allowed: {}", mul_match, allowed_to_mul);
        if allowed_to_mul {
            let (_, [first, second]) = mul_re.captures(mul_match.as_str()).unwrap().extract();
            let firstnum: i128 = first.parse().unwrap();
            let secondnum: i128 = second.parse().unwrap();
            numbers.push(firstnum * secondnum);
        } else {
            eprintln!("not allowed");
        }
    }

    let sum: i128 = numbers.iter().sum();
    sum.to_string()
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