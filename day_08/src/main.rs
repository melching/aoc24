use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn read_file() -> String {
    let content = fs::read_to_string("./input").expect("Something went wrong reading the file");
    return content;
}

fn get_antennas(input: &String) -> (HashMap<char, Vec<(i32, i32)>>, usize, usize) {
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for (y, row) in input.lines().enumerate() {
        for (x, col) in row.chars().enumerate() {
            if col == '.' {
                continue;
            }
            if !antennas.contains_key(&col) {
                antennas.insert(col, Vec::new());
            }
            antennas.get_mut(&col).unwrap().push((x as i32, y as i32))
        }
    }
    return (
        antennas,
        input.lines().count(),
        input.lines().next().unwrap().len(),
    );
}

fn calc_new_coords(pos1: (i32, i32), pos2: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    let diff_x = pos2.0 - pos1.0;
    let diff_y = pos2.1 - pos1.1;

    return (
        (pos1.0 - diff_x, pos1.1 - diff_y),
        (pos2.0 + diff_x, pos2.1 + diff_y),
    );
}

fn calc_new_coords_harmonic(
    pos1: (i32, i32),
    pos2: (i32, i32),
    height: i32,
    width: i32,
) -> Vec<(i32, i32)> {
    let mut antinodes: Vec<(i32, i32)> = Vec::new();

    let diff_x = pos2.0 - pos1.0;
    let diff_y = pos2.1 - pos1.1;

    // im too lazy to make it clean, just loop twice
    let mut x = pos1.0;
    let mut y = pos1.1;

    while coord_is_valid((x, y), height, width) {
        antinodes.push((x, y));
        x += diff_x;
        y += diff_y;
    }

    let mut x = pos2.0;
    let mut y = pos2.1;
    while coord_is_valid((x, y), height, width) {
        antinodes.push((x, y));
        x -= diff_x;
        y -= diff_y;
    }

    return antinodes;
}

fn coord_is_valid(pos: (i32, i32), height: i32, width: i32) -> bool {
    return !(pos.0 < 0 || pos.0 >= width || pos.1 < 0 || pos.1 >= height);
}

fn get_antinodes(
    antennas: &HashMap<char, Vec<(i32, i32)>>,
    height: i32,
    width: i32,
    harmonic: bool,
) -> Vec<(i32, i32)> {
    let mut antinodes: Vec<(i32, i32)> = Vec::new();

    for (_, positions) in antennas.iter() {
        // for each pair, check if position is valid
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                // check if antinode possible due to coords
                let pos1: (i32, i32) = positions[i];
                let pos2: (i32, i32) = positions[j];

                if !harmonic {
                    let (an1, an2) = calc_new_coords(pos1, pos2);
                    for an in [an1, an2] {
                        if coord_is_valid(an, height, width) && !antinodes.contains(&an) {
                            antinodes.push(an);
                        }
                    }
                } else {
                    let ans = calc_new_coords_harmonic(pos1, pos2, height, width);
                    for an in ans {
                        if !antinodes.contains(&an) {
                            antinodes.push(an);
                        }
                    }
                }
            }
        }
    }

    return antinodes;
}

fn main() {
    let now = Instant::now();
    let content: String = read_file();

    // part one
    let (antennas, height, width) = get_antennas(&content);
    let antinodes = get_antinodes(&antennas, height as i32, width as i32, false);
    println!("Anti-Nodes: {:?}", antinodes.len());

    // part two
    let antinodes = get_antinodes(&antennas, height as i32, width as i32, true);
    println!("Anti-Nodes (harmonic): {:?}", antinodes.len());

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_content() -> String {
        "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
            .to_string()
    }

    #[test]
    fn test_solution() {
        let content = get_test_content();
        let (antennas, height, width) = get_antennas(&content);
        println!(
            "Antennas: {:?}, height: {}, width: {}",
            antennas, height, width
        );
        let antinodes = get_antinodes(&antennas, height as i32, width as i32, false);
        println!("Nodes: {:?}", antinodes);

        assert_eq!(antinodes.len(), 14);
    }

    #[test]
    fn test_solution_2() {
        let content = get_test_content();
        let (antennas, height, width) = get_antennas(&content);
        println!(
            "Antennas: {:?}, height: {}, width: {}",
            antennas, height, width
        );
        let antinodes = get_antinodes(&antennas, height as i32, width as i32, true);
        println!("Nodes: {:?}", antinodes);

        assert_eq!(antinodes.len(), 34);
    }
}
