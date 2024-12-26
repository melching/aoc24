use std::fs;
use std::time::Instant;

const HEIGHT: i32 = 5;

fn read_file() -> String {
    let content = fs::read_to_string("./input").expect("Something went wrong reading the file");
    return content;
}

fn parse_input(input: &String) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut keys: Vec<Vec<i32>> = Vec::new();
    let mut locks: Vec<Vec<i32>> = Vec::new();

    let mut split = input.split("\n\n");
    loop {
        let block = split.next();
        match block {
            None => break,
            Some(b) => {
                let lines: Vec<String> = b.split("\n").map(|x| x.parse().unwrap()).collect();
                let mut new_block = vec![-1; 5];
                for line in lines.iter() {
                    for (i, c) in line.chars().enumerate() {
                        if c == '#' {
                            new_block[i] += 1;
                        }
                    }
                }

                if b.starts_with("#####") {
                    keys.push(new_block);
                } else {
                    locks.push(new_block);
                }
            }
        }
    }

    (keys, locks)
}

fn is_match(key: &Vec<i32>, lock: &Vec<i32>) -> bool {
    assert_eq!(key.len(), lock.len());
    for i in 0..key.len() {
        if key[i] + lock[i] > HEIGHT {
            return false;
        }
    }
    return true;
}

fn try_all(keys: &Vec<Vec<i32>>, locks: &Vec<Vec<i32>>) -> Vec<(Vec<i32>, Vec<i32>)> {
    let mut working_combinations: Vec<(Vec<i32>, Vec<i32>)> = Vec::new();
    for key in keys.iter() {
        for lock in locks.iter() {
            if is_match(key, lock) {
                working_combinations.push((key.clone(), lock.clone()));
            }
        }
    }
    return working_combinations;
}

fn main() {
    let input: String = read_file();
    let now = Instant::now();

    // part one
    let (locks, keys) = parse_input(&input);
    let combs = try_all(&keys, &locks);
    println!("Number of combinations: {}", combs.len());
    // part two

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_content_1() -> (String, usize) {
        (
            "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"
                .to_string(),
            3,
        )
    }

    #[test]
    fn test_solution_1() {
        let (input, solution) = get_test_content_1();
        let (locks, keys) = parse_input(&input);
        println!("locks: {:?}, keys: {:?}", locks, keys);
        let combs = try_all(&keys, &locks);

        assert_eq!(combs.len(), solution);
    }
}
