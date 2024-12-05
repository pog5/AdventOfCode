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
    let mut page_rules: Vec<(i32, i32)> = Vec::new();;
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
            eprintln!("update: {:?}", update);
            valid_updates.push(update);
        }
    }).collect::<Vec<_>>();

    let middle_elements = valid_updates.iter().map(|update| update[update.len() / 2]).collect::<Vec<i32>>();
    let sum: i32 = middle_elements.iter().sum();

    eprintln!("page_pairs: {:?}", page_rules);
    eprintln!("updates: {:?}", updates);
    sum.to_string()
}

fn part2(input: &str) -> String {
    "TODO".to_string()
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
