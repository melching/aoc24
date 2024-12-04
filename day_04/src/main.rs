use std::fs;
use std::time::Instant;

// idea for part one: read rows, columes and diagonals as separate string and the do a simple string search.
// lets see if part two in compatible with this approach.
// ... Now that ive read part two, yeah, lets do part two differently.

fn read_file() -> String {
    let content = fs::read_to_string("./input").expect("Something went wrong reading the file");
    return content;
}

pub fn get_matrix(content: &String) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for l in content.lines() {
        let row: Vec<char> = l.chars().collect();
        matrix.push(row);
    }
    return matrix;
}

pub fn parse_string(content: &String) -> Vec<String> {
    let mut strings: Vec<String> = Vec::new();

    let matrix = get_matrix(content);

    // rows
    strings.extend(matrix.iter().map(|x| x.iter().collect::<String>()));
    // columns
    strings.extend((0..matrix[0].len()).map(|i| matrix.iter().map(|x| x[i]).collect::<String>()));

    fn get_diagonals_new(input: &Vec<Vec<char>>) -> Vec<String> {
        let mut diag_strings: Vec<String> = Vec::new();
        let width = input[0].len();
        let height = input.len();

        // first go right to left, then top to bottom
        for i in 0..width {
            let mut diag: Vec<char> = Vec::new();
            let mut temp_w = i;
            let mut temp_h = 0;
            while temp_w < width && temp_h < height {
                diag.push(input[temp_h][temp_w]);
                temp_w += 1;
                temp_h += 1;
            }
            diag_strings.push(diag.iter().collect::<String>());
        }

        for i in 1..height {
            let mut diag: Vec<char> = Vec::new();
            let mut temp_w = 0;
            let mut temp_h = i;
            while temp_w < width && temp_h < height {
                diag.push(input[temp_h][temp_w]);
                temp_w += 1;
                temp_h += 1;
            }
            diag_strings.push(diag.iter().collect::<String>());
        }

        return diag_strings;
    }

    strings.extend(get_diagonals_new(&matrix));
    let mut matrix_reversed_rows: Vec<Vec<char>> = matrix.iter().map(|x| x.clone()).collect();
    for row in matrix_reversed_rows.iter_mut() {
        row.reverse();
    }
    strings.extend(get_diagonals_new(&matrix_reversed_rows));
    return strings;
}

pub fn count_xmases(strings: Vec<String>) -> usize {
    let mut count: usize = 0;
    for s in strings {
        count += s.matches("XMAS").count();
        count += s.matches("SAMX").count();
    }
    return count;
}

pub fn count_x_mases(matrix: &Vec<Vec<char>>) -> usize {
    let mut count: usize = 0;
    for i in 1..matrix.len() - 1 {
        for j in 1..matrix[0].len() - 1 {
            if matrix[i][j] != 'A' {
                continue;
            }
            // make diag top left to bottom right, then the other reverse
            let diag_1: String = vec![matrix[i - 1][j - 1], matrix[i][j], matrix[i + 1][j + 1]]
                .iter()
                .collect();
            let diag_2: String = vec![matrix[i + 1][j - 1], matrix[i][j], matrix[i - 1][j + 1]]
                .iter()
                .collect();

            if ["MAS", "SAM"].contains(&diag_1.as_str())
                && ["MAS", "SAM"].contains(&diag_2.as_str())
            {
                count += 1;
            }
        }
    }
    return count;
}

fn main() {
    let now = Instant::now();

    let file_content: String = read_file();
    // part one
    let strings = parse_string(&file_content);
    let count = count_xmases(strings);
    println!("Count: {}", count);

    // part two
    let matrix = get_matrix(&file_content);
    let x_count = count_x_mases(&matrix);
    println!("X-Count: {}", x_count);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let content: String = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            .to_string();
        let count = count_xmases(parse_string(&content));
        assert_eq!(count, 18);
    }

    #[test]
    fn test_solution_2() {
        let content: String = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            .to_string();
        let matrix = get_matrix(&content);
        let count = count_x_mases(&matrix);
        assert_eq!(count, 9);
    }
}
