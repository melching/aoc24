use std::fs;
use std::time::Instant;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
#[derive(Debug, PartialEq, Clone, Copy)]
enum FieldState {
    EMPTY,
    OBSTACLE,
    VISITED,
}

// what do we want to save? I'd like to have a matrix where I can mark each field and have a
// list of position the guard was already in, as this will likely be good
fn make_field(content: &String) -> (Vec<Vec<(FieldState, Vec<Direction>)>>, (usize, usize)) {
    let mut field: Vec<Vec<(FieldState, Vec<Direction>)>> = Vec::new();
    let mut start: (usize, usize) = (0, 0);

    for (i, line) in content.lines().enumerate() {
        let mut row: Vec<(FieldState, Vec<Direction>)> = Vec::new();
        for (j, char) in line.chars().enumerate() {
            match char {
                '#' => row.push((FieldState::OBSTACLE, vec![])),
                '.' => row.push((FieldState::EMPTY, vec![])),
                '^' => {
                    row.push((FieldState::VISITED, vec![Direction::UP]));
                    start = (j, i)
                }
                _ => panic!("This should never happen"),
            }
        }
        field.push(row);
    }

    return (field, start);
}

fn mark_visited(field: &mut (FieldState, Vec<Direction>), dir: Direction) {
    // let mut new_state = field.clone();
    match field.0 {
        FieldState::OBSTACLE => panic!("You should not be able to land here"),
        FieldState::EMPTY => {
            field.0 = FieldState::VISITED;
            field.1.push(dir)
        }
        FieldState::VISITED => field.1.push(dir),
    }
}

fn count_visited(field: &Vec<Vec<(FieldState, Vec<Direction>)>>) -> i32 {
    let mut count = 0;
    for row in field {
        for col in row {
            if col.0 == FieldState::VISITED {
                count += 1;
            }
        }
    }
    count
}

fn run_patrol(field: &mut Vec<Vec<(FieldState, Vec<Direction>)>>, start: (usize, usize)) -> bool {
    let width = field[0].len();
    let height = field.len();

    let mut pos = start.clone();
    let mut direction: Direction = Direction::UP;
    let mut found_loop = false;
    while true {
        // run until we break. Also not trying out to specify the loops in case I need more
        // make a move
        match direction {
            Direction::UP => {
                // oob
                if pos.1 == 0 {
                    break;
                }
                // obstacle
                if FieldState::OBSTACLE == field[pos.1 - 1][pos.0].0 {
                    direction = Direction::RIGHT;
                } else {
                    // free, so lets move
                    pos.1 -= 1;
                }
            }
            Direction::DOWN => {
                if pos.1 == height - 1 {
                    break;
                }
                if FieldState::OBSTACLE == field[pos.1 + 1][pos.0].0 {
                    direction = Direction::LEFT;
                } else {
                    pos.1 += 1;
                }
            }
            Direction::LEFT => {
                if pos.0 == 0 {
                    break;
                }
                if FieldState::OBSTACLE == field[pos.1][pos.0 - 1].0 {
                    direction = Direction::UP;
                } else {
                    pos.0 -= 1;
                }
            }
            Direction::RIGHT => {
                if pos.0 == width - 1 {
                    break;
                }
                if FieldState::OBSTACLE == field[pos.1][pos.0 + 1].0 {
                    direction = Direction::DOWN;
                } else {
                    pos.0 += 1;
                }
            }
        }

        if field[pos.1][pos.0].1.contains(&direction) {
            // already visited, were done as we are in a loop
            found_loop = true;
            break;
        }
        mark_visited(&mut field[pos.1][pos.0], direction.clone())
    }
    found_loop
}

fn read_file() -> String {
    let content = fs::read_to_string("./input").expect("Something went wrong reading the file");
    return content;
}
fn main() {
    let now = Instant::now();
    let content: String = read_file();

    // part one
    let (clean_field, start) = make_field(&content);
    let mut field = clean_field.clone();
    run_patrol(&mut field, start);

    let result = count_visited(&field);
    println!("Count: {}", result);

    // part two
    let mut loops = 0;
    for (y, row) in field.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            // we dont need to place where we never will be
            if col.0 != FieldState::VISITED {
                continue;
            }

            // dont check start position
            if (x, y) == start {
                continue;
            }

            let mut temp_field = clean_field.clone();
            temp_field[y][x].0 = FieldState::OBSTACLE;
            if run_patrol(&mut temp_field, start) {
                loops += 1;
            }
        }
    }
    println!("Loops: {}", loops);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_content() -> String {
        "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            .to_string()
    }

    #[test]
    fn test_solution() {
        let content = get_test_content();

        let (mut field, start) = make_field(&content);
        run_patrol(&mut field, start);

        let result = count_visited(&field);
        assert_eq!(result, 41);
    }

    #[test]
    fn test_solution_2() {
        let content = get_test_content();

        let (clean_field, start) = make_field(&content);
        let mut field: Vec<Vec<(FieldState, Vec<Direction>)>> = clean_field.clone();
        run_patrol(&mut field, start);

        let mut loops = 0;
        for (y, row) in field.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                // we dont need to place where we never will be
                if col.0 != FieldState::VISITED {
                    continue;
                }

                // dont check start position
                if (x, y) == start {
                    continue;
                }

                let mut new_field = clean_field.clone();
                new_field[y][x].0 = FieldState::OBSTACLE;
                if run_patrol(&mut new_field, start) {
                    loops += 1;
                }
            }
        }

        assert_eq!(loops, 6);
    }
}
