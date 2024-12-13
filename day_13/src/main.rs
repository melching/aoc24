use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

fn read_file() -> String {
    let content = fs::read_to_string("./input").expect("Something went wrong reading the file");
    return content;
}

fn get_machine_configs(input: &String) -> Vec<((i64, i64), (i64, i64), (i64, i64))> {
    let mut configs: Vec<((i64, i64), (i64, i64), (i64, i64))> = Vec::new();
    for config_str in input.split("\n\n") {
        let mut line_iter = config_str.lines();

        fn get_two_numbers(line: &str) -> (i64, i64) {
            let re = Regex::new(r"(\d+)").unwrap();
            re.captures_iter(line)
                .map(|x| x[1].parse::<i64>().expect("Not a number"))
                .collect_tuple()
                .unwrap()
        }

        // line 1
        // Format: Button A: X+94, Y+34
        let btn_a: (i64, i64) = get_two_numbers(line_iter.next().unwrap());
        // line 2
        let btn_b: (i64, i64) = get_two_numbers(line_iter.next().unwrap());
        // line 3
        let target: (i64, i64) = get_two_numbers(line_iter.next().unwrap());

        configs.push((btn_a, btn_b, target));
    }
    configs
}

fn is_solvable_gcd(btn_a: (i64, i64), btn_b: (i64, i64), target: (i64, i64)) -> bool {
    let mut x = target.0;
    let mut y = target.1;
    let mut found = false;
    while x >= 0 && y >= 0 {
        if x % btn_a.0 == 0 && y % btn_a.1 == 0 {
            found = true;
            break;
        }
        x -= btn_b.0;
        y -= btn_b.1;
    }
    found
}

// brute force
fn find_lowest_price(btn_a: (i64, i64), btn_b: (i64, i64), target: (i64, i64), limit: bool) -> i64 {
    if !is_solvable_gcd(btn_a, btn_b, target) {
        return i64::MAX;
    }

    let mut price: i64 = i64::MAX;
    let max = if limit { 101 } else { i64::MAX };
    'a_loop: for fa in 1..max {
        if btn_a.0 * fa > target.0 || btn_a.1 * fa > target.1 {
            break;
        }
        for fb in 1..max {
            if btn_b.0 * fb > target.0 || btn_b.1 * fb > target.1 {
                continue 'a_loop;
            }
            if btn_a.0 * fa + btn_b.0 * fb == target.0
                && btn_a.1 * fa + btn_b.1 * fb == target.1
                && fa * 3 + fb < price
            {
                price = fa * 3 + fb;
            }
        }
    }
    price
}

// Try another approach. Try the cheapest possible solution first and then increase the price.
// First check for x and if a solution for x is found test y.
// Turns out this is barely faster (if any) but does not rely on an upper limit of iterations.
fn find_lowest_price_faster(a: (i64, i64), b: (i64, i64), t: (i64, i64)) -> i64 {
    let mut fa: i64 = t.0 / a.0;

    if !is_solvable_gcd(a, b, t) {
        return i64::MAX;
    }

    fn solves_y(a: (i64, i64), b: (i64, i64), t: (i64, i64), fa: i64, fb: i64) -> bool {
        t.1 == a.1 * fa + b.1 * fb
    }

    while fa >= 0 {
        let rem: i64 = t.0 - a.0 * fa;
        // check if it solves the x coord
        if rem % b.0 == 0 {
            // found solution for x, check if it also solves y
            let fb = rem / b.0;
            if solves_y(a, b, t, fa, fb) {
                return fa * 3 + fb;
            }
        }
        fa -= 1;
    }

    i64::MAX
}

// I think i am stupid, lets just solve the equations by hand and see if we can find a pattern
// tx = x0 * a + x1 * b
// ty = y0 * a + y1 * b
// tx * y1 = x0 * a * y1 + x1 * b * y1
// ty * x1 = x1 * a * y0 + x1 * b * y1
// tx * y1 - ty * x1 = x0 * a * y1 + x1 * b * y1 - x1 * a * y0 - x1 * b * y1
// tx * y1 - ty * x1 = x0 * a * y1 - x1 * a * y0
// tx * y1 - ty * x1 = a * (x0 * y1 - x1 * y0)
// a = (tx * y1 - ty * x1) / (x0 * y1 - x1 * y0)

// now lets find b, also I do not know why I did not do it like this to begin with
// tx = x0 * a + x1 * b
// tx - x0 * a = x1 * b
// b = (tx - x0 * a) / x1

fn find_lowest_price_faster_now_for_real(a: (i64, i64), b: (i64, i64), t: (i64, i64)) -> i64 {
    // need to check if results are integer, as otherwise we have a solutions that is not allowed
    if (t.0 * b.1 - t.1 * b.0) % (a.0 * b.1 - b.0 * a.1) != 0 {
        return i64::MAX;
    }
    let fa = (t.0 * b.1 - t.1 * b.0) / (a.0 * b.1 - b.0 * a.1); // (t0 * y1 - t1 * x1) / (x0 * y1 - x1 * y0)

    if (t.0 - a.0 * fa) % b.0 != 0 {
        return i64::MAX;
    }
    let fb = (t.0 - a.0 * fa) / b.0; // (t0 - x0 * a) / x1
    return fa * 3 + fb;
}

fn main() {
    let input: String = read_file();
    let configs = get_machine_configs(&input);

    let now = Instant::now();
    // part one
    let mut sum = 0;
    for config in &configs {
        let price = find_lowest_price_faster_now_for_real(config.0, config.1, config.2);
        if price != i64::MAX {
            sum += price;
        }
    }
    println!("Sum: {}", sum);
    // part two
    let mut sum = 0;
    for config in &configs {
        let price = find_lowest_price_faster_now_for_real(
            config.0,
            config.1,
            (config.2 .0 + 10000000000000, config.2 .1 + 10000000000000),
        );
        if price != i64::MAX {
            sum += price;
        }
    }
    println!("Sum + 10x: {}", sum);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_content() -> String {
        "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"
            .to_string()
    }

    #[test]
    fn test_solution() {
        let input = get_test_content();
        let configs = get_machine_configs(&input);
        let mut sum = 0;
        for config in configs {
            let price = find_lowest_price_faster_now_for_real(config.0, config.1, config.2);
            println!("Price: {}, config {:?}", price, config);
            if price != i64::MAX {
                sum += price;
            }
        }

        assert_eq!(sum, 480);
    }

    #[test]
    fn test_solution_2() {
        let input = get_test_content();
        let configs = get_machine_configs(&input);
        let mut sum = 0;
        for config in configs {
            let price = find_lowest_price_faster_now_for_real(
                config.0,
                config.1,
                (config.2 .0 + 10000000000000, config.2 .1 + 10000000000000),
            );
            println!("Price: {}, config {:?}", price, config);
            if price != i64::MAX {
                sum += price;
            }
        }
        println!("Sum: {}", sum);
    }
}
