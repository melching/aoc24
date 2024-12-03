use std::fs;
use std::time::Instant;

// copied from day 1, adjusted for day 2
fn read_and_parse_file() -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let content = fs::read_to_string("./input").expect("Something went wrong reading the file");

    for line in content.lines() {
        let split: Vec<&str> = line.split(" ").collect();
        let mut row: Vec<i32> = Vec::new();

        if split.len() > 0 {
            for s in split {
                let number: i32 = s.parse().expect("Not a number");
                row.push(number);
            }
            result.push(row);
        }
    }
    return result;
}

fn is_safe(values: &Vec<i32>, try_dampened: bool) -> bool {
    if values.len() <= 1 {
        return true;
    }
    let increasing: bool = values[0] < values[1];

    for i in 1..values.len() {
        let diff: i32 = values[i] - values[i - 1];
        if diff == 0 || diff.abs() > 3 || (diff < 0 && increasing || diff > 0 && !increasing) {
            if try_dampened {
                for j in 0..values.len() {
                    let mut rem: Vec<i32> = values.clone();
                    rem.remove(j);
                    if is_safe(&rem, false) {
                        return true;
                    }
                }

                // this does not work and I dont know why
                // let mut rem_1: Vec<i32> = values.clone();
                // let mut rem_2: Vec<i32> = values.clone();
                // rem_1.remove(i - 1);
                // rem_2.remove(i);
                // assert_eq!(rem_1.len(), rem_2.len());
                // let is_safe_first = is_safe(&rem_1, false);
                // let is_safe_second = is_safe(&rem_2, false);
                // return is_safe_first || is_safe_second;
            }
            return false;
        }
    }
    return true;
}

fn main() {
    let now = Instant::now();

    let values = read_and_parse_file();

    // part one and two
    let mut safe_rows: i32 = 0;
    let mut safe_rows_dampened: i32 = 0;
    for row in values.iter() {
        if is_safe(row, false) {
            safe_rows += 1;
        } else if is_safe(row, true) {
            safe_rows_dampened += 1;
        }
    }
    println!("Safe rows: {}", safe_rows);
    println!("Safe rows dampened: {}", safe_rows_dampened + safe_rows);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
