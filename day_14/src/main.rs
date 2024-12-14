use itertools::Itertools;
use regex::Regex;
use std::fs;
use std::time::Instant;

fn read_file() -> String {
    let content = fs::read_to_string("./input").expect("Something went wrong reading the file");
    return content;
}

fn get_robots(input: &String) -> Vec<((i64, i64), (i64, i64))> {
    let mut configs: Vec<((i64, i64), (i64, i64))> = Vec::new();
    for config_str in input.lines() {
        fn get_four_numbers(line: &str) -> (i64, i64, i64, i64) {
            let re = Regex::new(r"(-?\d+)").unwrap();
            re.captures_iter(line)
                .map(|x| x[1].parse::<i64>().expect("Not a number"))
                .collect_tuple()
                .unwrap()
        }
        let nums = get_four_numbers(config_str);
        configs.push(((nums.0, nums.1), (nums.2, nums.3)));
    }
    configs
}

fn move_robot(pos: (i64, i64), vel: (i64, i64), dims: (i64, i64)) -> (i64, i64) {
    // return !(pos.0 < 0 || pos.0 >= dims.0 as i64 || pos.1 < 0 || pos.1 >= dims.1 as i64);
    let mut new_pos = (pos.0 + vel.0, pos.1 + vel.1);
    new_pos.0 = (new_pos.0 + dims.0) % dims.0;
    new_pos.1 = (new_pos.1 + dims.1) % dims.1;
    new_pos
}
//dims (11, 7) => , pos (6,6)
fn get_quadrant(pos: (i64, i64), dims: (i64, i64)) -> Option<usize> {
    if pos.0 == dims.0 / 2 || pos.1 == dims.1 / 2 {
        return None;
    }
    let mut qd: (usize, usize) = (0, 0);
    if pos.0 <= dims.0 / 2 {
        qd.0 += 1;
    }
    if pos.1 <= dims.1 / 2 {
        qd.1 += 1;
    }
    return Some(qd.0 + qd.1 * 2);
}

fn run_seconds(robots: &mut Vec<((i64, i64), (i64, i64))>, dims: (i64, i64), seconds: usize) {
    for _ in 0..seconds {
        for robot in robots.iter_mut() {
            robot.0 = move_robot(robot.0, robot.1, dims);
        }
    }
}

fn calc_quadrants(robots: Vec<((i64, i64), (i64, i64))>, dims: (i64, i64)) -> usize {
    let mut quadrants: Vec<usize> = vec![0; 4];
    for robot in robots.iter() {
        let q_v = get_quadrant(robot.0, dims);
        match q_v {
            Some(x) => {
                quadrants[x] += 1;
            }
            None => {}
        }
    }
    if quadrants.iter().sum::<usize>() == 0 {
        return 0;
    }
    let mut prod: usize = 1;
    for q in quadrants {
        prod *= q
    }
    return prod;
}

fn render_robots(robots: &Vec<((i64, i64), (i64, i64))>, dims: (i64, i64)) {
    let mut matrix: Vec<Vec<char>> = vec![vec![' '; dims.0 as usize]; dims.1 as usize];
    for robot in robots {
        matrix[robot.0 .1 as usize][robot.0 .0 as usize] = '#'; // lets ignore the count for now
    }
    for row in matrix.iter() {
        println!("{}", row.iter().collect::<String>())
    }
}

fn main() {
    let input: String = read_file();
    let robots = get_robots(&input);

    let now = Instant::now();

    // part one
    let dims = (101, 103);
    let mut robots_1 = robots.clone();
    run_seconds(&mut robots_1, dims, 100);
    let product = calc_quadrants(robots_1, dims);
    println!("Product of quadrants {}", product);

    // part two (wtf)
    let mut robots_2 = robots.clone();
    // As I dont have an idea how the tree would look like, I have no good idea to check programmatically.
    // I thought about calculating the density of robots (assuming the tree is filles),
    // but that was likely thought of and there is probably another cluster at the same time.
    // Maybe just render it nicely and go step by step - lets hope the image is not in the ten-thousands.
    // Update: Just dump everything into a file and search for certain pattern (#####) using ctrl+f. Found the tree after second 6377.
    for second in 1..10000 {
        // love a good while true loop
        println!("State after second {}", second);
        run_seconds(&mut robots_2, dims, 1);
        render_robots(&robots_2, dims);
        println!("{}", vec!['-'; dims.0 as usize].iter().collect::<String>());
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_content() -> String {
        "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"
            .to_string()
    }

    #[test]
    fn test_solution() {
        let input = get_test_content();
        let mut robots = get_robots(&input);
        run_seconds(&mut robots, (11, 7), 100);
        let product = calc_quadrants(robots, (11, 7));

        assert_eq!(product, 12);
    }

    #[test]
    fn test_solution_2() {}
}
