use std::fs;
use std::time::Instant;

fn read_file() -> String {
    let content = fs::read_to_string("./input").expect("Something went wrong reading the file");
    return content;
}

fn build_warehouse(input: &String) -> (Vec<Vec<char>>, (usize, usize), Vec<char>) {
    let mut split = input.split("\n\n");

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut robot_pos: (usize, usize) = (0, 0); // will be changed
    for (y, line) in split.next().unwrap().lines().enumerate() {
        map.push(line.chars().collect());
        if map[y].contains(&'@') {
            let x = map[y].iter().position(|&x| x == '@').unwrap();
            robot_pos = (x, y);
        }
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

fn sum_gps(map: &Vec<Vec<char>>) -> usize {
    let mut sum: usize = 0;
    for (y, line) in map.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if char == &'O' {
                sum += y * 100 + x;
            }
        }
    }
    sum
}

fn main() {
    let now = Instant::now();
    let input: String = read_file();
    let (mut map, mut pos, moves) = build_warehouse(&input);
    // part one
    make_moves(&mut map, &mut pos, &moves);
    let sum = sum_gps(&map);
    println!("Sum: {}", sum);

    // part two

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
        let (mut map, mut pos, moves) = build_warehouse(&input);
        make_moves(&mut map, &mut pos, &moves);
        assert_eq!(sum_gps(&map), 10092);
    }

    #[test]
    fn test_solution_1_2() {
        let input = get_test_content_2();
        let (mut map, mut pos, moves) = build_warehouse(&input);
        make_moves(&mut map, &mut pos, &moves);
        assert_eq!(sum_gps(&map), 2028);
    }
}
