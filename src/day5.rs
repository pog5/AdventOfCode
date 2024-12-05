fn is_update_valid(update: Vec<i32>, print_rules: Vec<(i32, i32)>) -> bool {
    let mut flag = true;

    for (before, after) in print_rules {
        if update.contains(&before) && update.contains(&after) {
            let before_index = update.iter().position(|&x| x == before).unwrap();
            let after_index = update.iter().position(|&x| x == after).unwrap();

            if before_index > after_index {
                flag = false;
                break;
            }
        }
    }

    flag
}

fn part1(input: &str) -> String {
    let mut page_rules: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    let _ = input.lines()
        .map(|line| {
            if line.contains("|") {
                let mut pages = line.split('|');
                let page = pages.next();
                if page.is_some() {
                    let page2 = pages.next();
                    if page2.is_some() {
                        let parsedfirst = page.unwrap().parse::<i32>().unwrap();
                        let parsedsecond = page2.unwrap().parse::<i32>().unwrap();
                        page_rules.push((parsedfirst, parsedsecond));
                    }
                }
            } else if line.contains(",") {
                let update_split = line.split(',');
                let parsed: Vec<i32> = update_split.map(|x| x.parse::<i32>().unwrap()).collect();
                updates.push(parsed);
            }
        }).collect::<Vec<_>>();

    let mut valid_updates = Vec::new();
    let _ = updates.iter().map(|update| {
        if is_update_valid(update.clone(), page_rules.clone()) {
            valid_updates.push(update);
        }
    }).collect::<Vec<_>>();

    let middle_elements = valid_updates.iter().map(|update| update[update.len() / 2]).collect::<Vec<i32>>();
    let sum: i32 = middle_elements.iter().sum();

    sum.to_string()
}

fn fix_update(update: &Vec<i32>, print_rules: Vec<(i32, i32)>) -> Vec<i32> {
    let mut fixed_update = update.clone();
    let mut changed = true;

    while changed {
        changed = false;
        for &(before, after) in &print_rules {
            if fixed_update.contains(&before) && fixed_update.contains(&after) {
                let before_index = fixed_update.iter().position(|&x| x == before).unwrap();
                let after_index = fixed_update.iter().position(|&x| x == after).unwrap();

                if before_index > after_index {
                    fixed_update.swap(before_index, after_index);
                    changed = true;
                    break;
                }
            }
        }
    }

    fixed_update
}

fn part2(input: &str) -> String {
    let mut page_rules: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    let _ = input.lines()
        .map(|line| {
            if line.contains("|") {
                let mut pages = line.split('|');
                let page = pages.next();
                if page.is_some() {
                    let page2 = pages.next();
                    if page2.is_some() {
                        let parsedfirst = page.unwrap().parse::<i32>().unwrap();
                        let parsedsecond = page2.unwrap().parse::<i32>().unwrap();
                        page_rules.push((parsedfirst, parsedsecond));
                    }
                }
            } else if line.contains(",") {
                let update_split = line.split(',');
                let parsed: Vec<i32> = update_split.map(|x| x.parse::<i32>().unwrap()).collect();
                updates.push(parsed);
            }
        }).collect::<Vec<_>>();

    let mut fixed_updates = Vec::new();
    let _ = updates.iter().map(|update| {
        if !is_update_valid(update.clone(), page_rules.clone()) {
            fixed_updates.push(fix_update(update, page_rules.clone()));
        }
    }).collect::<Vec<_>>();

    let middle_elements = fixed_updates.iter().map(|update| update[update.len() / 2]).collect::<Vec<i32>>();
    let sum: i32 = middle_elements.iter().sum();

    sum.to_string()
    // 4705, 4653 -> too high
}

pub fn day5() {
    let input = include_str!("../input/day5.txt");
    let response = part1(input);
    println!();
    let response2 = part2(input);

    println!();
    println!("Part One: {}", response);
    println!("Part Two: {}", response2);
}
