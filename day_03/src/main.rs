use regex::Regex;
use std::fs;
use std::time::Instant;

fn read_and_parse_file() -> String {
    let content = fs::read_to_string("./input").expect("Something went wrong reading the file");
    return content;
}

fn get_mults(content: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut tuples: Vec<(i32, i32)> = Vec::new();
    for (_, [x, y]) in re.captures_iter(content).map(|x| x.extract()) {
        tuples.push((
            x.parse().expect("Not a number"),
            y.parse().expect("Not a number"),
        ));
    }
    return tuples;
}

fn main() {
    let now = Instant::now();

    let content: String = read_and_parse_file();

    // part one
    let tuples = get_mults(&content);
    let mut result: i32 = 0;
    for (x, y) in tuples {
        result += x * y;
    }
    println!("Sum of mults: {}", result);

    // part two
    let mut new_tuples: Vec<(i32, i32)> = Vec::new();

    let mut remaining: String = content.clone();
    let mut next_instruction: &str = "don't()";
    while remaining.len() > 0 {
        let index: Option<usize> = remaining.find(next_instruction);
        match index {
            Some(i) => {
                let (before, after) = remaining.split_at(i);
                if next_instruction == "don't()" {
                    next_instruction = "do()";
                    new_tuples.extend(get_mults(before));
                } else {
                    next_instruction = "don't()";
                }
                remaining = after.to_string();
            }
            None => {
                if next_instruction == "don't()" {
                    new_tuples.extend(get_mults(&remaining));
                }
                break;
            }
        }
    }
    let mut new_result: i32 = 0;
    for (x, y) in new_tuples {
        new_result += x * y;
    }
    println!("Sum of mults with dos and donts: {}", new_result);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
