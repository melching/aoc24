use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)] // lets see how much i can stuff in here even though i don't need it.
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn read_file() -> String {
    let content = fs::read_to_string("./input").expect("Something went wrong reading the file");
    return content;
}

fn build_garden(input: &String) -> Vec<Vec<char>> {
    let mut garden: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for char in line.chars() {
            row.push(char);
        }
        garden.push(row);
    }
    garden
}

fn coord_is_valid(dims: (usize, usize), pos: (i64, i64)) -> bool {
    return !(pos.0 < 0 || pos.0 >= dims.0 as i64 || pos.1 < 0 || pos.1 >= dims.1 as i64);
}

fn find_field(
    garden: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    curr: (usize, usize),
    dims: (usize, usize),
) -> (usize, usize) {
    if visited[curr.1][curr.0] {
        return (0, 0); // already tracked
    }
    visited[curr.1][curr.0] = true;
    let mut area: usize = 1;
    let mut perimeter: usize = 4;
    for change in vec![(0, -1), (1, 0), (0, 1), (-1, 0)] {
        let new_pos_signed = (curr.0 as i64 + change.0, curr.1 as i64 + change.1);
        if !coord_is_valid(dims, new_pos_signed) {
            continue;
        }
        let new_pos = (new_pos_signed.0 as usize, new_pos_signed.1 as usize);
        if garden[curr.1][curr.0] != garden[new_pos.1][new_pos.0] {
            continue;
        }
        let update = find_field(garden, visited, new_pos, dims);
        area += update.0;
        perimeter += update.1;
        perimeter -= 1; // we found an edge that is valid, so we have to reduce us
    }
    (area, perimeter)
}

fn find_field_2(
    garden: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<(bool, HashSet<Direction>)>>,
    curr: (usize, usize),
    dims: (usize, usize),
) -> (usize, usize) {
    if visited[curr.1][curr.0].0 {
        return (0, 0); // already tracked
    }
    visited[curr.1][curr.0].0 = true;
    let mut area: usize = 1;
    let mut sides: usize = 0;

    let mut local_known_sides: HashSet<Direction> = HashSet::new();
    let mut adjacent_known_sides: HashMap<Direction, usize> = HashMap::new();

    for (change, direction) in vec![
        ((0, -1), Direction::UP),
        ((1, 0), Direction::RIGHT),
        ((0, 1), Direction::DOWN),
        ((-1, 0), Direction::LEFT),
    ] {
        let new_pos_signed = (curr.0 as i64 + change.0, curr.1 as i64 + change.1);
        if !coord_is_valid(dims, new_pos_signed) {
            local_known_sides.insert(direction);
            continue;
        }
        let new_pos = (new_pos_signed.0 as usize, new_pos_signed.1 as usize);
        if garden[curr.1][curr.0] != garden[new_pos.1][new_pos.0] {
            local_known_sides.insert(direction);
            continue;
        }
        let update = find_field_2(garden, visited, new_pos, dims);

        area += update.0;
        sides += update.1;

        if vec![Direction::UP, Direction::DOWN].contains(&direction) {
            for _dir in vec![Direction::LEFT, Direction::RIGHT] {
                if visited[new_pos.1][new_pos.0].1.contains(&_dir) {
                    *adjacent_known_sides.entry(_dir).or_insert(0) += 1;
                }
            }
        }
        if vec![Direction::LEFT, Direction::RIGHT].contains(&direction) {
            for _dir in vec![Direction::UP, Direction::DOWN] {
                if visited[new_pos.1][new_pos.0].1.contains(&_dir) {
                    *adjacent_known_sides.entry(_dir).or_insert(0) += 1;
                }
            }
        }
    }

    for side in &local_known_sides {
        sides += 1;
        if adjacent_known_sides.contains_key(side) {
            sides -= adjacent_known_sides.get(side).unwrap();
        }
    }
    visited[curr.1][curr.0].1 = local_known_sides;
    (area, sides)
}

fn get_results(garden: &Vec<Vec<char>>) -> HashMap<u64, (usize, usize)> {
    let height = garden.len();
    let width = garden[0].len();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; width]; height];
    let mut field_idx: u64 = 0;
    let mut results: HashMap<u64, (usize, usize)> = HashMap::new();

    for (y, row) in garden.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if visited[y][x] {
                continue;
            }
            let result: (usize, usize);
            result = find_field(garden, &mut visited, (x, y), (width, height));
            results.insert(field_idx, result);
            field_idx += 1;
        }
    }
    results
}

fn get_results_2(garden: &Vec<Vec<char>>) -> HashMap<u64, (usize, usize)> {
    let height = garden.len();
    let width = garden[0].len();
    let mut visited: Vec<Vec<(bool, HashSet<Direction>)>> =
        vec![vec![(false, HashSet::new()); width]; height];
    let mut field_idx: u64 = 0;
    let mut results: HashMap<u64, (usize, usize)> = HashMap::new();

    for (y, row) in garden.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if visited[y][x].0 {
                continue;
            }
            let result = find_field_2(garden, &mut visited, (x, y), (width, height));
            results.insert(field_idx, result);
            field_idx += 1;
        }
    }
    results
}

fn get_cost(info: &HashMap<u64, (usize, usize)>) -> usize {
    let mut total: usize = 0;
    for (_, v) in info {
        total += v.0 * v.1
    }
    total
}

fn main() {
    let now = Instant::now();
    let input: String = read_file();
    let garden = build_garden(&input);

    // part one
    let results = get_results(&garden);
    let cost = get_cost(&results);
    println!("Cost: {}", cost);
    // part two
    let results = get_results_2(&garden);
    let cost = get_cost(&results);
    println!("Cost reduced: {}", cost);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_content() -> String {
        "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
            .to_string()
    }

    #[test]
    fn test_solution() {
        let input = get_test_content();
        let garden = build_garden(&input);
        let results = get_results(&garden);
        println!("{:?}", results);
        let cost = get_cost(&results);
        assert_eq!(cost, 1930);
    }

    #[test]
    fn test_solution_2() {
        let input = get_test_content();
        let garden = build_garden(&input);
        let results = get_results_2(&garden);
        println!("Part Two: {:?}", results);
        let cost = get_cost(&results);
        assert_eq!(cost, 1206);
    }

    #[test]
    fn test_solution_2_1() {
        let input = "A".to_string();
        let garden = build_garden(&input);
        let results = get_results_2(&garden);
        println!("Part Two: {:?}", results);
        let cost = get_cost(&results);
        assert_eq!(cost, 4);
    }
    #[test]
    fn test_solution_2_2() {
        let input = "AA".to_string();
        let garden = build_garden(&input);
        let results = get_results_2(&garden);
        println!("Part Two: {:?}", results);
        let cost = get_cost(&results);
        assert_eq!(cost, 8);
    }
    #[test]
    fn test_solution_2_2_1() {
        let input = "AA
AB"
        .to_string();
        let garden = build_garden(&input);
        let results = get_results_2(&garden);
        let cost = get_cost(&results);
        assert_eq!(cost, 18 + 4);
    }
    #[test]
    fn test_solution_2_3() {
        let input = "AAA
ABA"
        .to_string();
        let garden = build_garden(&input);
        let results = get_results_2(&garden);
        let cost = get_cost(&results);
        assert_eq!(cost, 40 + 4);
    }
    #[test]
    fn test_solution_2_4() {
        let input = "AA
AB
AA
BA
AA
AA"
        .to_string();
        let garden = build_garden(&input);
        let results = get_results_2(&garden);
        let cost = get_cost(&results);
        assert_eq!(cost, 10 * 12 + 2 * 4);
    }
}
