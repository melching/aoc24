use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn read_file() -> String {
    let content = fs::read_to_string("./input").expect("Something went wrong reading the file");
    return content;
}

fn get_initial_arrangement(input: &String) -> Vec<u64> {
    return input
        .split_whitespace()
        .map(|x| x.parse().expect("Not a number"))
        .collect();
}

// good old brute force did not work
fn blink(state: Vec<u64>) -> Vec<u64> {
    let mut new_state: Vec<u64> = Vec::new();

    for stone in state {
        if stone == 0 {
            new_state.push(1);
            continue;
        }
        let stone_str = stone.to_string();
        if stone_str.len() % 2 == 0 {
            let nums = stone_str.split_at(stone_str.len() / 2);
            new_state.push(nums.0.parse::<u64>().expect("NaN"));
            new_state.push(nums.1.parse::<u64>().expect("NaN"));
            continue;
        }
        new_state.push(stone * 2024);
    }
    new_state
}

fn blink_often_maybe_faster(input: &Vec<u64>, iterations: usize) -> usize {
    let mut total_count = 0;

    // make initial map
    let mut stones: HashMap<u64, usize> = HashMap::new();
    for stone in input {
        if !stones.contains_key(&stone) {
            stones.insert(*stone, 1);
        } else {
            *stones.get_mut(&stone).unwrap() += 1;
        }
    }

    for _ in 0..iterations {
        let mut new_stones: HashMap<u64, usize> = HashMap::new();
        for (stone, count) in stones.iter() {
            if *stone == 0 {
                *new_stones.entry(1).or_insert(0) += count;
                continue;
            }

            let stone_str = stone.to_string();
            if stone_str.len() % 2 == 0 {
                let nums = stone_str.split_at(stone_str.len() / 2);
                let parsed_nums = vec![
                    nums.0.parse::<u64>().expect("NaN"),
                    nums.1.parse::<u64>().expect("NaN"),
                ];
                *new_stones.entry(parsed_nums[0]).or_insert(0) += count;
                *new_stones.entry(parsed_nums[1]).or_insert(0) += count;
                continue;
            }
            *new_stones.entry(stone * 2024).or_insert(0) += count;
        }
        stones = new_stones;
    }
    for stone in stones {
        total_count += stone.1;
    }

    return total_count;
}

fn main() {
    let now = Instant::now();
    let input: String = read_file();

    let start_state = get_initial_arrangement(&input);

    // part one
    let count_25 = blink_often_maybe_faster(&start_state, 25);
    println!("Number of stones (25): {}", count_25);

    // part two
    let count_75 = blink_often_maybe_faster(&start_state, 75);
    println!("Number of stones (75): {}", count_75);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_content() -> String {
        "125 17".to_string()
    }

    #[test]
    fn test_solution() {
        let input = get_test_content();
        let mut state = get_initial_arrangement(&input);
        for _ in 0..25 {
            state = blink(state);
        }

        assert_eq!(state.len(), 55312);
    }
    #[test]
    fn test_solution_1_5() {
        let input = get_test_content();
        let state = get_initial_arrangement(&input);
        let count = blink_often_maybe_faster(&state, 25);

        assert_eq!(count, 55312);
    }

    #[test]
    fn test_solution_2() {
        let input = get_test_content();
        let state = get_initial_arrangement(&input);
        let count = blink_often_maybe_faster(&state, 75);
        println!("{}", count);
    }
}
