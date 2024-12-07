use std::fs;
use std::time::Instant;

fn read_file() -> String {
    let content = fs::read_to_string("./input").expect("Something went wrong reading the file");
    return content;
}

fn parse_content(content: &String) -> Vec<(i64, Vec<i64>)> {
    let mut results: Vec<(i64, Vec<i64>)> = Vec::new();

    for line in content.lines() {
        let split: Vec<&str> = line.split(":").collect();
        assert!(split.len() == 2);

        let eq_result: i64 = split[0].parse().expect("Not a number, weird");
        let eq_numbers: Vec<i64> = split[1]
            .split_whitespace()
            .map(|x| x.parse().expect("Not a number"))
            .collect();
        results.push((eq_result, eq_numbers));
    }

    return results;
}

fn search_for_solution(target: i64, last: i64, next: &[i64], new_operator: bool) -> bool {
    if next.len() == 0 {
        return target == last;
    }
    if next.iter().max().unwrap() > &target {
        return false;
    }

    let new_last_add = last + next[0];
    if new_last_add <= target {
        if search_for_solution(target, new_last_add, &next[1..], new_operator) {
            return true;
        }
    }

    let new_last_mult = last * next[0];
    if new_last_mult <= target {
        // * is valid
        if search_for_solution(target, new_last_mult, &next[1..], new_operator) {
            return true;
        }
    }
    if new_operator {
        let str_concat = last.to_string() + &next[0].to_string();
        let new_value: i64 = str_concat.parse().expect("Not a number with new operator");
        if new_value <= target {
            if search_for_solution(target, new_value, &next[1..], new_operator) {
                return true;
            }
        }
    }
    return false;
}

fn main() {
    let now = Instant::now();
    let content: String = read_file();

    // part one
    let equations = parse_content(&content);

    let mut sum_valid = 0;
    for eq in &equations {
        if search_for_solution(eq.0, eq.1[0], &eq.1[1..], false) {
            sum_valid += eq.0;
        }
    }
    println!("Sum of valid eqs (+,*): {}", sum_valid);

    // part two
    let mut sum_valid = 0;
    for eq in &equations {
        if search_for_solution(eq.0, eq.1[0], &eq.1[1..], true) {
            sum_valid += eq.0;
        }
    }
    println!("Sum of valid eqs (+,*,||): {}", sum_valid);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_content() -> String {
        "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
            .to_string()
    }

    #[test]
    fn test_solution() {
        let content = get_test_content();
        let equations = parse_content(&content);

        let mut sum_valid = 0;
        for eq in equations {
            if search_for_solution(eq.0, eq.1[0], &eq.1[1..], false) {
                sum_valid += eq.0;
            }
        }

        assert_eq!(sum_valid, 3749);
    }
    #[test]
    fn test_solution_2() {
        let content = get_test_content();
        let equations = parse_content(&content);

        let mut sum_valid = 0;
        for eq in equations {
            if search_for_solution(eq.0, eq.1[0], &eq.1[1..], true) {
                sum_valid += eq.0;
            }
        }

        assert_eq!(sum_valid, 11387);
    }
}
