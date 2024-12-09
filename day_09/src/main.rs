use std::cmp::Ordering;
use std::fs;
use std::time::Instant;

fn read_file() -> String {
    let content = fs::read_to_string("./input").expect("Something went wrong reading the file");
    return content;
}

fn parse_input(input: &String) -> Vec<(i64, u64)> {
    let mut list: Vec<(i64, u64)> = Vec::new();

    let mut idx: i64 = 0;
    let mut is_file = true;
    for count in input.chars().map(|x| x.to_digit(10).expect("Not a number")) {
        if count == 0 {
            is_file = !is_file;
            continue;
        }
        if !is_file {
            list.push((-1, count as u64))
        } else {
            list.push((idx, count as u64));
            idx += 1;
        }

        is_file = !is_file;
    }
    return list;
}

fn defrag_in_place(input: &mut Vec<(i64, u64)>) {
    let mut curr: usize = 0;
    let mut end: usize = input.len() - 1;
    while curr < end {
        if input[curr].0 != -1 {
            curr += 1;
            continue;
        }
        if input[end].0 == -1 {
            end -= 1;
            continue;
        }
        match input[end].1.cmp(&input[curr].1) {
            Ordering::Equal => {
                input[curr].0 = input[end].0;
                input[end].0 = -1;
            }
            Ordering::Less => {
                input.insert(curr, input[end]);
                input[curr + 1].1 -= input[curr].1;
                input[end + 1].0 = -1;
            }
            Ordering::Greater => {
                input[curr].0 = input[end].0;
                input[end].1 -= input[curr].1;
                input.insert(end + 1, (-1, input[curr].1));
            }
        }
    }
}
fn defrag_blocks_in_place(input: &mut Vec<(i64, u64)>) {
    let mut end: usize = input.len() - 1;
    while end - 1 > 0 {
        if input[end].0 == -1 {
            end -= 1;
            continue;
        }
        'curr_loop: for curr in 0..end {
            if input[curr].0 != -1 {
                continue;
            }
            match input[end].1.cmp(&input[curr].1) {
                Ordering::Equal => {
                    input[curr].0 = input[end].0;
                    input[end].0 = -1;
                    break 'curr_loop;
                }
                Ordering::Less => {
                    input.insert(curr, input[end]);
                    input[curr + 1].1 -= input[curr].1;
                    input[end + 1].0 = -1;
                    end += 1;
                    break 'curr_loop;
                }
                Ordering::Greater => {
                    continue;
                }
            }
        }
        end -= 1;
    }
}

fn make_total(input: &Vec<(i64, u64)>) -> i64 {
    let mut total: i64 = 0;
    let mut idx: i64 = 0;
    for elem in input {
        if elem.0 == -1 {
            idx += elem.1 as i64;
            continue;
        }
        for _ in 0..elem.1 {
            total += idx * elem.0;
            idx += 1;
        }
    }
    return total;
}

fn main() {
    let now = Instant::now();
    let content: String = read_file();

    let input = parse_input(&content);
    // part one
    let mut input_one = input.clone();

    defrag_in_place(&mut input_one);
    let total = make_total(&input_one);
    println!("Total: {}", total);

    // part two
    let mut input_two = input.clone();

    defrag_blocks_in_place(&mut input_two);
    let total = make_total(&input_two);
    println!("Total (with blocky only): {}", total);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_content() -> String {
        "2333133121414131402".to_string()
    }

    #[test]
    fn test_solution() {
        let content = get_test_content();
        let mut input = parse_input(&content);

        defrag_in_place(&mut input);
        let total = make_total(&input);

        println!("Total: {}", total);

        assert_eq!(total, 1928);
    }

    #[test]
    fn test_solution_2() {
        let content = get_test_content();
        let mut input = parse_input(&content);

        defrag_blocks_in_place(&mut input);
        let total = make_total(&input);

        println!("Total: {}", total);

        assert_eq!(total, 2858);
    }
}
