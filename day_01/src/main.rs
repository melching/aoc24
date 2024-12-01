use std::fs;

fn read_and_parse_file() -> (Vec<i32>, Vec<i32>) {
    let mut first: Vec<i32> = Vec::new();
    let mut second: Vec<i32> = Vec::new();

    let content = fs::read_to_string("./input").expect("Something went wrong reading the file");

    for line in content.lines() {
        let split: Vec<&str> = line.split("   ").collect();

        let first_number: i32 = split[0].parse().expect("Not a number");
        let second_number: i32 = split[1].parse().expect("Not a number");
        first.push(first_number);
        second.push(second_number);
    }
    return (first, second);
}

fn main() {
    let (first, second) = read_and_parse_file();

    // part one
    let mut first_sorted: Vec<i32> = first.clone();
    first_sorted.sort();
    let mut second_sorted: Vec<i32> = second.clone();
    second_sorted.sort();

    assert_eq!(first_sorted.len(), second_sorted.len());

    let mut sum: i32 = 0;
    for i in 0..first_sorted.len() {
        let diff = (first_sorted[i] - second_sorted[i]).abs();
        sum += diff;
    }

    println!("Sum of differences: {}", sum);

    //part two
    let mut sim_score: i32 = 0;
    for v in first.iter() {
        let count: i32 = second.iter().filter(|&x| x == v).count() as i32;
        sim_score += v * count;
    }

    println!("Similarity score: {}", sim_score);
}
