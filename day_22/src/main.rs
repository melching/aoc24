use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn read_file() -> String {
    let content = fs::read_to_string("./input").expect("Something went wrong reading the file");
    return content;
}

fn get_initial_numbers(input: &String) -> Vec<usize> {
    return input
        .lines()
        .map(|x| x.parse::<usize>().expect("Not a positive number"))
        .collect();
}

fn mix(secret: usize, value: usize) -> usize {
    return secret ^ value;
}

fn prune(value: usize) -> usize {
    return value % 16777216;
}

fn step(secret: usize) -> usize {
    let mut new_secret = secret;

    // step 1
    let temp = secret * 64;
    let mixed = mix(secret, temp);
    new_secret = prune(mixed);

    // step 2
    let temp = new_secret / 32;
    let mixed = mix(new_secret, temp);
    new_secret = prune(mixed);

    // step 3
    let temp = new_secret * 2048;
    let mixed = mix(new_secret, temp);
    new_secret = prune(mixed);

    return new_secret;
}

fn step_times(secret: usize, steps: usize) -> usize {
    let mut new_secret = secret;
    for _ in 0..steps {
        new_secret = step(new_secret);
    }
    new_secret
}

fn get_last_digit(value: usize) -> isize {
    let last_char = value.to_string().pop().unwrap();
    return last_char.to_digit(10).expect("Not a number?!") as isize;
}

fn step_times_get_sequences(secret: usize, steps: usize) -> HashMap<Vec<isize>, isize> {
    // track what sets of diff we have already seen
    let mut seen: HashMap<Vec<isize>, isize> = HashMap::new();

    let mut new_secret = secret;
    let mut last_digit_old = get_last_digit(new_secret);

    // last four diffs
    let mut diff_last: Vec<isize> = vec![last_digit_old as isize; 4];

    for i in 0..steps {
        new_secret = step(new_secret);

        let last_digit_new = get_last_digit(new_secret);
        diff_last.remove(0);
        diff_last.push(last_digit_new - last_digit_old);

        last_digit_old = last_digit_new;

        if i >= 3 && !seen.contains_key(&diff_last) {
            seen.insert(diff_last.clone(), last_digit_old);
        }
    }
    seen
}

fn main() {
    let input: String = read_file();
    let numbers = get_initial_numbers(&input);

    let now = Instant::now();

    // part one
    let mut total = 0;
    for n in numbers {
        total += step_times(n, 2000);
    }
    println!("Sum of new secrets: {}", total);

    // part two
    let numbers = get_initial_numbers(&input);
    let mut totals: HashMap<Vec<isize>, isize> = HashMap::new();
    for n in numbers.iter() {
        let sequences = step_times_get_sequences(*n, 2000);
        for (k, v) in sequences.iter() {
            *totals.entry(k.clone()).or_default() += v;
        }
    }

    // get max value
    let mut key_max: Vec<isize> = vec![0; 4];
    let mut max: isize = 0;
    for (k, v) in totals.iter() {
        if v > &max {
            key_max = k.clone();
            max = *v;
        }
    }

    println!(
        "Found max amount of bananes at sequence {:?}: {}",
        key_max, max
    );

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_content() -> String {
        "1
10
100
2024"
            .to_string()
    }

    fn get_test_content_2() -> String {
        "1
2
3
2024"
            .to_string()
    }

    #[test]
    fn test_mix() {
        assert_eq!(mix(42, 15), 37)
    }

    #[test]
    fn test_prune() {
        assert_eq!(prune(100000000), 16113920)
    }

    #[test]
    fn test_step() {
        assert_eq!(step(123), 15887950);
        assert_eq!(step_times(123, 10), 5908254);
    }

    #[test]
    fn test_solution() {
        let input = get_test_content();
        let numbers = get_initial_numbers(&input);
        assert_eq!(step_times(numbers[0], 2000), 8685429);
        assert_eq!(step_times(numbers[1], 2000), 4700978);
        assert_eq!(step_times(numbers[2], 2000), 15273692);
        assert_eq!(step_times(numbers[3], 2000), 8667524);
    }

    #[test]
    fn test_solution_2() {
        let input = get_test_content_2();
        let numbers = get_initial_numbers(&input);

        let mut totals: HashMap<Vec<isize>, isize> = HashMap::new();
        for n in numbers.iter() {
            let sequences = step_times_get_sequences(*n, 2000);
            for (k, v) in sequences.iter() {
                *totals.entry(k.clone()).or_default() += v;
            }
        }

        // get max value
        let mut key_max: Vec<isize> = vec![0; 4];
        let mut max: isize = 0;
        for (k, v) in totals.iter() {
            if v > &max {
                key_max = k.clone();
                max = *v;
            }
        }

        println!(
            "Found max amount of bananes at sequence {:?}: {}",
            key_max, max
        );

        assert_eq!(key_max, vec![-2, 1, -1, 3]);
        assert_eq!(max, 23);
    }
}
