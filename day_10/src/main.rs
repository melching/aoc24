use std::collections::HashSet;
use std::fs;
use std::time::Instant;

fn read_file() -> String {
    let content = fs::read_to_string("./input").expect("Something went wrong reading the file");
    return content;
}

// make a map (height of map, width of map, height, count) where count stores information how many valid finishes
// can be reached from here (-1 => unknown, 0 => no finishes, >0 => #finishes).
// This will likely make problems when part two comes, but lets see first.
// ---
// Quick update during part 1: I misread the challenge and counted all paths instead of all individual ends.
// But as I don't want to remove all this, lets just add it as well :)
// Therefore the structure is now (height of map, width of map, height of pos, count of paths to end, set of ends)
// ---
// Quick update when starting part 2: Turns out when i misread i solved pat two. Ez i guess.
fn get_map(input: &String) -> Vec<Vec<(u32, i32, HashSet<(i32, i32)>)>> {
    let mut map: Vec<Vec<(u32, i32, HashSet<(i32, i32)>)>> = Vec::new();

    for row in input.lines() {
        let mut new_row: Vec<(u32, i32, HashSet<(i32, i32)>)> = Vec::new();
        for col in row.chars() {
            new_row.push((col.to_digit(10).expect("Not a number"), -1, HashSet::new()));
        }
        map.push(new_row);
    }
    map
}

fn pos_is_valid(dims: (usize, usize), pos: (i32, i32)) -> bool {
    return !(pos.0 < 0 || pos.0 >= dims.0 as i32 || pos.1 < 0 || pos.1 >= dims.1 as i32);
}

// this function assumes we only check increasing values
fn check_trail_position(
    map: &mut Vec<Vec<(u32, i32, HashSet<(i32, i32)>)>>,
    dims: (usize, usize),
    pos: (i32, i32),
) {
    if map[pos.1 as usize][pos.0 as usize].0 == 9 {
        map[pos.1 as usize][pos.0 as usize].1 = 1;
        map[pos.1 as usize][pos.0 as usize].2.insert(pos);
        return; // TODO
    }
    map[pos.1 as usize][pos.0 as usize].1 = 0; // we start at 0
    for change in vec![(0, -1), (1, 0), (0, 1), (-1, 0)] {
        let new_pos = (pos.0 + change.0, pos.1 + change.1);
        if !(pos_is_valid(dims, new_pos)
            && map[pos.1 as usize][pos.0 as usize].0 + 1
                == map[new_pos.1 as usize][new_pos.0 as usize].0)
        {
            continue;
        }
        if map[new_pos.1 as usize][new_pos.0 as usize].1 == -1 {
            check_trail_position(map, dims, new_pos);
        }
        map[pos.1 as usize][pos.0 as usize].1 += map[new_pos.1 as usize][new_pos.0 as usize].1;
        let new_nines = map[new_pos.1 as usize][new_pos.0 as usize].2.clone();
        map[pos.1 as usize][pos.0 as usize]
            .2
            .extend(new_nines.iter());
    }
}

fn find_trail_sum(map: &mut Vec<Vec<(u32, i32, HashSet<(i32, i32)>)>>) -> (usize, usize) {
    let mut sum_ends = 0;
    let mut sum_routes: usize = 0;

    let height = map.len();
    let width = map[0].iter().len();
    for x in 0..width {
        for y in 0..height {
            if map[y][x].0 == 0 {
                check_trail_position(map, (width, height), (x as i32, y as i32));
                sum_ends += map[y][x].2.len();
                sum_routes += map[y][x].1 as usize;
            }
        }
    }
    (sum_ends, sum_routes)
}

fn main() {
    let now = Instant::now();
    let content: String = read_file();

    // part one
    let mut map = get_map(&content);
    let sums = find_trail_sum(&mut map);

    println!("Sum: {:?}", sums.0);
    // part two
    println!("Sum: {:?}", sums.1);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_content() -> String {
        "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
            .to_string()
    }

    #[test]
    fn test_solution() {
        let input = get_test_content();
        let mut map = get_map(&input);
        let sums = find_trail_sum(&mut map);
        for row in map {
            println!("{:?}", row);
        }

        assert_eq!(sums.0, 36);
    }
}
