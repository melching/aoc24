use std::collections::HashSet;
use std::fs;
use std::time::Instant;

fn read_file() -> String {
    let content = fs::read_to_string("./input").expect("Something went wrong reading the file");
    return content;
}

fn build_warehouse(input: &String, wide: bool) -> (Vec<Vec<char>>, (usize, usize), Vec<char>) {
    let mut split = input.split("\n\n");

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut robot_pos: (usize, usize) = (0, 0); // will be changed
    for (y, line) in split.next().unwrap().lines().enumerate() {
        let mut row: Vec<char> = Vec::new();
        for (x, char) in line.chars().enumerate() {
            if char == '@' {
                robot_pos = (x, y);
                if wide {
                    row.extend(vec!['@', '.']);
                    robot_pos.0 *= 2;
                } else {
                    row.push(char);
                }
            } else if char == 'O' {
                if wide {
                    row.extend(vec!['[', ']']);
                } else {
                    row.push(char);
                }
            } else {
                row.extend(vec![char; wide as usize + 1]);
            }
        }
        map.push(row);
    }

    let mut moves: Vec<char> = Vec::new();
    for line in split.next().unwrap().lines() {
        let chars = line.chars().collect::<Vec<char>>();
        moves.extend(chars);
    }
    (map, robot_pos, moves)
}

fn render_board(map: &Vec<Vec<char>>) {
    for row in map {
        println!("{}", row.iter().collect::<String>())
    }
}

fn make_moves(map: &mut Vec<Vec<char>>, pos: &mut (usize, usize), moves: &Vec<char>) {
    let height = map.len();
    let width = map.iter().len();
    for dir in moves {
        match dir {
            '^' => match map[pos.1 - 1][pos.0] {
                '.' => {
                    map[pos.1 - 1][pos.0] = '@';
                    map[pos.1][pos.0] = '.';
                    pos.1 -= 1;
                }
                'O' => {
                    for y in (1..pos.1).rev() {
                        if map[y][pos.0] == '.' {
                            map[y][pos.0] = 'O';
                            map[pos.1][pos.0] = '.';
                            map[pos.1 - 1][pos.0] = '@';
                            pos.1 -= 1;
                            break;
                        }
                        if map[y][pos.0] == '#' {
                            break;
                        }
                    }
                }
                _ => {}
            },
            'v' => match map[pos.1 + 1][pos.0] {
                '.' => {
                    map[pos.1 + 1][pos.0] = '@';
                    map[pos.1][pos.0] = '.';
                    pos.1 += 1;
                }
                'O' => {
                    for y in pos.1 + 2..height {
                        if map[y][pos.0] == '.' {
                            map[y][pos.0] = 'O';
                            map[pos.1][pos.0] = '.';
                            map[pos.1 + 1][pos.0] = '@';
                            pos.1 += 1;
                            break;
                        }
                        if map[y][pos.0] == '#' {
                            break;
                        }
                    }
                }
                _ => {}
            },
            '<' => match map[pos.1][pos.0 - 1] {
                '.' => {
                    map[pos.1][pos.0 - 1] = '@';
                    map[pos.1][pos.0] = '.';
                    pos.0 -= 1;
                }
                'O' => {
                    for x in (1..pos.0).rev() {
                        if map[pos.1][x] == '.' {
                            map[pos.1][x] = 'O';
                            map[pos.1][pos.0] = '.';
                            map[pos.1][pos.0 - 1] = '@';
                            pos.0 -= 1;
                            break;
                        }
                        if map[pos.1][x] == '#' {
                            break;
                        }
                    }
                }
                _ => {}
            },
            '>' => match map[pos.1][pos.0 + 1] {
                '.' => {
                    map[pos.1][pos.0 + 1] = '@';
                    map[pos.1][pos.0] = '.';
                    pos.0 += 1;
                }
                'O' => {
                    for x in pos.0 + 2..width {
                        if map[pos.1][x] == '.' {
                            map[pos.1][x] = 'O';
                            map[pos.1][pos.0] = '.';
                            map[pos.1][pos.0 + 1] = '@';
                            pos.0 += 1;
                            break;
                        }
                        if map[pos.1][x] == '#' {
                            break;
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}

fn is_movable_wide(
    map: &Vec<Vec<char>>,
    pos: &(usize, usize),
    dir: &char,
    to_move: &mut HashSet<(usize, usize)>,
) -> bool {
    let mut new_pos = pos.clone();
    let mut check_siblings = false;
    to_move.insert(*pos);
    match dir {
        '^' => {
            new_pos.1 -= 1;
            check_siblings = true;
        }
        '>' => new_pos.0 += 1,
        '<' => new_pos.0 -= 1,
        'v' => {
            new_pos.1 += 1;
            check_siblings = true;
        }
        _ => {}
    }

    match map[new_pos.1][new_pos.0] {
        '.' => {
            return true;
        }
        '#' => return false,
        '[' => {
            if check_siblings {
                return is_movable_wide(map, &(new_pos.0 + 1, new_pos.1), dir, to_move)
                    && is_movable_wide(map, &new_pos, dir, to_move);
            } else {
                return is_movable_wide(map, &new_pos, dir, to_move);
            }
        }
        ']' => {
            if check_siblings {
                return is_movable_wide(map, &(new_pos.0 - 1, new_pos.1), dir, to_move)
                    && is_movable_wide(map, &new_pos, dir, to_move);
            } else {
                return is_movable_wide(map, &new_pos, dir, to_move);
            }
        }
        _ => {}
    }
    false
}

fn move_wide(map: &mut Vec<Vec<char>>, pos: &mut (usize, usize), dir: &char) {
    let mut to_move: HashSet<(usize, usize)> = HashSet::new();
    let is_movable = is_movable_wide(map, &pos, dir, &mut to_move);

    if !is_movable {
        return;
    }

    let mut diff: (i32, i32) = (0, 0);
    match dir {
        '^' => diff.1 = -1,
        '>' => diff.0 = 1,
        '<' => diff.0 = -1,
        'v' => diff.1 = 1,
        _ => {}
    }

    let old_map = map.clone();
    to_move.insert(*pos);
    for update_pos in to_move.iter() {
        map[update_pos.1][update_pos.0] = '.';
    }
    for update_pos in to_move.iter() {
        map[(update_pos.1 as i32 + diff.1) as usize][(update_pos.0 as i32 + diff.0) as usize] =
            old_map[update_pos.1][update_pos.0];
    }
    pos.0 = (pos.0 as i32 + diff.0) as usize;
    pos.1 = (pos.1 as i32 + diff.1) as usize;
}

fn make_moves_wide(map: &mut Vec<Vec<char>>, pos: &mut (usize, usize), moves: &Vec<char>) {
    for dir in moves {
        move_wide(map, pos, dir);
    }
}

fn sum_gps(map: &Vec<Vec<char>>, wide: bool) -> usize {
    let mut sum: usize = 0;
    for (y, line) in map.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if !wide && char == &'O' {
                sum += y * 100 + x;
            }
            if wide && char == &'[' {
                sum += y * 100 + x;
            }
        }
    }
    sum
}

fn main() {
    let now = Instant::now();
    let input: String = read_file();

    // part one
    let (mut map, mut pos, moves) = build_warehouse(&input, false);
    make_moves(&mut map, &mut pos, &moves);
    let sum = sum_gps(&map, false);
    println!("Sum: {}", sum);

    // part two
    let (mut map, mut pos, moves) = build_warehouse(&input, true);
    make_moves_wide(&mut map, &mut pos, &moves);
    let sum = sum_gps(&map, true);
    println!("Sum: {}", sum);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_content_1() -> String {
        "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
            .to_string()
    }

    fn get_test_content_2() -> String {
        "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"
            .to_string()
    }

    #[test]
    fn test_solution_1_1() {
        let input = get_test_content_1();
        let (mut map, mut pos, moves) = build_warehouse(&input, false);
        make_moves(&mut map, &mut pos, &moves);
        assert_eq!(sum_gps(&map, false), 10092);
    }

    #[test]
    fn test_solution_1_2() {
        let input = get_test_content_2();
        let (mut map, mut pos, moves) = build_warehouse(&input, false);
        make_moves(&mut map, &mut pos, &moves);
        assert_eq!(sum_gps(&map, false), 2028);
    }

    #[test]
    fn test_solution_2() {
        let input = get_test_content_1();
        let (mut map, mut pos, moves) = build_warehouse(&input, true);
        make_moves_wide(&mut map, &mut pos, &moves);
        assert_eq!(sum_gps(&map, true), 9021);
    }

    // #[test]
    // fn test_solution_2_2() {
    //     let input = get_test_content_2();
    //     let (mut map, mut pos, moves) = build_warehouse(&input, true);
    //     make_moves_wide(&mut map, &mut pos, &moves);
    //     assert_eq!(sum_gps(&map, true), 9021);
    // }
}
