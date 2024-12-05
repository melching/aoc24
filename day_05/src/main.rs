use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn read_file() -> String {
    let content = fs::read_to_string("./input").expect("Something went wrong reading the file");
    return content;
}

fn get_rules_and_updates(content: &String) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();

    let content_split = content.split("\n\n").collect::<Vec<&str>>();
    assert_eq!(content_split.len(), 2);

    for l in content_split[0].lines() {
        let numbers: Vec<u32> = l
            .split("|")
            .map(|x| x.parse().expect("Not a number"))
            .collect();
        assert_eq!(numbers.len(), 2);

        if rules.contains_key(&numbers[0]) && !rules.get(&numbers[0]).unwrap().contains(&numbers[1])
        // dont know sets yet, so jsut check for duplicates
        {
            rules.get_mut(&numbers[0]).unwrap().push(numbers[1]);
        } else {
            rules.insert(numbers[0], vec![numbers[1]]);
        }
    }

    for l in content_split[1].lines() {
        let numbers: Vec<u32> = l
            .split(",")
            .map(|x| x.parse().expect("Not a number"))
            .collect();
        assert!(numbers.len() > 0);
        updates.push(numbers);
    }

    (rules, updates)
}

fn is_update_valid(rules: &HashMap<u32, Vec<u32>>, update: &Vec<u32>) -> bool {
    for i in 1..update.len() {
        if !rules.contains_key(&update[i]) {
            continue;
        }
        for j in 0..i {
            if rules.get(&update[i]).unwrap().contains(&update[j]) {
                return false;
            }
        }
    }
    return true;
}

fn get_sum_of_updates(updates: &Vec<Vec<u32>>) -> u32 {
    let mut sum: u32 = 0;
    for update in updates {
        sum += update[update.len() / 2] as u32;
    }
    return sum;
}

// I implemented this first because I did not think properly about the problem, see next function for better solution
// Spoiler: it takes also significantly longer! (almost 30s)
fn sort_invalid_update_by_rules(rules: &HashMap<u32, Vec<u32>>, update: &Vec<u32>) -> Vec<u32> {
    // this assumes that rules cannot be contradicting
    let mut valid_update: Vec<u32> = update.clone();

    let mut i: usize = 1;
    while i < update.len() {
        if !rules.contains_key(&valid_update[i]) {
            i += 1;
            continue;
        }
        let rule = rules.get(&valid_update[i]).unwrap();

        let mut current: usize = 0;
        let mut stop: usize = i;
        while current < stop {
            if rule.contains(&valid_update[current]) {
                let value = valid_update[current];
                valid_update.remove(current);
                valid_update.insert(i, value);
                stop -= 1;
                i -= 1;
            } else {
                current += 1;
            }
        }
        i += 1;
    }
    assert_eq!(valid_update.len(), update.len());
    assert!(is_update_valid(rules, &valid_update));
    return valid_update;
}

fn sort_invalid_update_by_rules_but_better(
    rules: &HashMap<u32, Vec<u32>>,
    update: &Vec<u32>,
) -> Vec<u32> {
    let mut valid_update: Vec<u32> = update.clone();
    valid_update.sort_by(|a, b| {
        if !rules.contains_key(a) {
            return std::cmp::Ordering::Equal;
        } else if rules.get(a).unwrap().contains(b) {
            return std::cmp::Ordering::Less;
        } else {
            return std::cmp::Ordering::Equal; // Was 'less' before, but given that we can't make the decision here I changed it to 'equal'
        }
    });
    return valid_update;
}

fn main() {
    let now = Instant::now();

    let file_content: String = read_file();
    let (rules, updates) = get_rules_and_updates(&file_content);

    // part one
    let mut valid_updates: Vec<Vec<u32>> = Vec::new();
    for update in &updates {
        if is_update_valid(&rules, update) {
            valid_updates.push(update.clone());
        }
    }
    let sum = get_sum_of_updates(&valid_updates);
    println!("Sum of valid: {}", sum);

    // part two
    let mut invalid_updates: Vec<Vec<u32>> = Vec::new();
    for update in &updates {
        if !is_update_valid(&rules, update) {
            invalid_updates.push(update.clone());
        }
    }
    let mut fixed_updates: Vec<Vec<u32>> = Vec::new();
    for update in invalid_updates {
        fixed_updates.push(sort_invalid_update_by_rules_but_better(&rules, &update));
    }
    let sum = get_sum_of_updates(&fixed_updates);
    println!("Sumof invalid: {}", sum);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_content() -> String {
        "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            .to_string()
    }

    #[test]
    fn test_solution() {
        let content = get_test_content();
        let (rules, updates) = get_rules_and_updates(&content);

        let mut valid_updates: Vec<Vec<u32>> = Vec::new();
        for update in &updates {
            if is_update_valid(&rules, update) {
                valid_updates.push(update.clone());
            }
        }
        let sum = get_sum_of_updates(&valid_updates);
        assert_eq!(sum, 143);
    }

    #[test]
    fn test_solution_2() {
        let content = get_test_content();
        let (rules, updates) = get_rules_and_updates(&content);
        let mut invalid_updates: Vec<Vec<u32>> = Vec::new();
        for update in &updates {
            if !is_update_valid(&rules, update) {
                invalid_updates.push(update.clone());
            }
        }
        let mut fixed_updates: Vec<Vec<u32>> = Vec::new();
        for update in invalid_updates {
            fixed_updates.push(sort_invalid_update_by_rules_but_better(&rules, &update));
        }
        let sum = get_sum_of_updates(&fixed_updates);
        assert_eq!(sum, 123);
    }
}
