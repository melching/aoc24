use cached::proc_macro::cached;
use cached::SizedCache;
use std::fs;
use std::time::Instant;
use tqdm::tqdm;

fn read_file() -> String {
    let content = fs::read_to_string("./input").expect("Something went wrong reading the file");
    return content;
}

fn get_towels_and_designs(input: &String) -> (Vec<&str>, Vec<&str>) {
    let split: Vec<&str> = input.split("\n\n").collect();
    let towels = split[0].split(", ").collect();

    let designs: Vec<&str> = split[1].lines().collect();
    (towels, designs)
}

#[cached(
    ty = "SizedCache<String, bool>",
    create = "{ SizedCache::with_size(1000) }",
    convert = r#"{ format!("{:?}{}", towels, design) }"#
)]
fn can_be_solved(towels: &Vec<&str>, design: &str) -> bool {
    // test all towels
    if design.len() == 0 {
        return true;
    }
    for towel in towels {
        if design.starts_with(towel) && can_be_solved(towels, design.strip_prefix(towel).unwrap()) {
            return true;
        }
    }
    false
}

#[cached(
    ty = "SizedCache<String, usize>",
    create = "{ SizedCache::with_size(1000) }",
    convert = r#"{ format!("{:?}{}", towels, design) }"#
)]
fn can_be_solved_n_ways(towels: &Vec<&str>, design: &str) -> usize {
    // test all towels
    if design.len() == 0 {
        return 1;
    }
    let mut solution_count = 0;
    for towel in towels {
        if design.starts_with(towel) {
            solution_count += can_be_solved_n_ways(towels, design.strip_prefix(towel).unwrap());
        }
    }
    solution_count
}

fn main() {
    let input: String = read_file();
    let (towels, designs) = get_towels_and_designs(&input);
    let now = Instant::now();

    // part one
    let mut sum = 0;
    for design in tqdm(designs.iter()) {
        if can_be_solved(&towels, design) {
            sum += 1;
        }
    }
    println!("Number of designs that can be solved {}", sum);

    // part two
    let mut total = 0;
    for design in tqdm(designs.iter()) {
        total += can_be_solved_n_ways(&towels, design);
    }
    println!("Number of solutions that can solve designs {}", total);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_content() -> String {
        "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
            .to_string()
    }

    #[test]
    fn test_solution() {
        let input = get_test_content();
        let (towels, designs) = get_towels_and_designs(&input);

        assert!(can_be_solved(&towels, designs[0]));
        assert!(can_be_solved(&towels, designs[1]));
        assert!(can_be_solved(&towels, designs[2]));
        assert!(can_be_solved(&towels, designs[3]));
        assert!(!can_be_solved(&towels, designs[4]));
        assert!(can_be_solved(&towels, designs[5]));
        assert!(can_be_solved(&towels, designs[6]));
        assert!(!can_be_solved(&towels, designs[7]));
    }

    #[test]
    fn test_solution_2() {
        let input = get_test_content();
        let (towels, designs) = get_towels_and_designs(&input);

        let mut total = 0;
        for design in designs {
            total += can_be_solved_n_ways(&towels, design);
        }
        assert_eq!(total, 16);
    }
}
