use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn read_file() -> String {
    let content = fs::read_to_string("./input").expect("Something went wrong reading the file");
    return content;
}

fn parse_input(
    input: &String,
) -> (
    HashMap<String, bool>,
    Vec<((String, String), String, String)>,
) {
    let mut initial: HashMap<String, bool> = HashMap::new();
    let mut gates: Vec<((String, String), String, String)> = Vec::new(); // connected, operand, output

    let mut split = input.split("\n\n");

    // parse initial values
    for line in split.next().unwrap().lines() {
        let mut line_split = line.split(": ");
        initial.insert(
            line_split.next().unwrap().parse().unwrap(),
            line_split.next().unwrap() == "1",
        );
    }

    // parse gates
    let re = Regex::new(r"(.+) (XOR|OR|AND) (.+) -> (.+)$").unwrap();
    for line in split.next().unwrap().lines() {
        for (_, [input1, operand, input2, output]) in re.captures_iter(line).map(|c| c.extract()) {
            gates.push((
                (input1.parse().unwrap(), input2.parse().unwrap()),
                operand.parse().unwrap(),
                output.parse().unwrap(),
            ));
        }
    }
    (initial, gates)
}

fn calc_gate(input1: &bool, input2: &bool, operand: &String) -> bool {
    match operand.as_str() {
        "AND" => return *input1 && *input2,
        "OR" => return *input1 || *input2,
        "XOR" => return *input1 ^ *input2,
        _ => panic!("You should not land here"),
    }
}

fn run<'a>(
    initial_states: &HashMap<String, bool>,
    gates: &Vec<((String, String), String, String)>,
) -> HashMap<String, bool> {
    let mut states = initial_states.clone();
    let mut remaining_gates = gates.clone();

    while remaining_gates.len() > 0 {
        let mut processed_gates: Vec<usize> = Vec::new();
        'rem_loop: for (i, gate) in remaining_gates.iter().enumerate() {
            if !states.contains_key(&gate.0 .0) || !states.contains_key(&gate.0 .1) {
                continue 'rem_loop;
            }
            let output = calc_gate(
                states.get(&gate.0 .0).unwrap(),
                states.get(&gate.0 .1).unwrap(),
                &gate.1,
            );
            states.insert(gate.2.clone(), output);
            processed_gates.push(i);
        }
        for (i, idx) in processed_gates.iter().enumerate() {
            remaining_gates.remove(idx - i);
        }
    }
    states
}

fn calc_result(states: &HashMap<String, bool>, starting_with: &str) -> usize {
    // get z-states
    let mut zstates: Vec<(String, bool)> = Vec::new();
    for state in states {
        if state.0.starts_with(&starting_with) {
            zstates.push((state.0.clone(), *state.1));
        }
    }
    zstates.sort_by(|a, b| a.0.cmp(&b.0));

    let mut out: usize = 0;
    for (i, (_, bit)) in zstates.iter().enumerate() {
        if *bit {
            out += 2_usize.pow(i as u32);
        }
    }
    out
}

fn check_for_loop(output: &String, gates: &Vec<((String, String), String, String)>) -> bool {
    let mut inputs_to_check: Vec<String> = Vec::new();
    let mut seen_outputs: Vec<String> = Vec::new();

    for gate in gates {
        if gate.0 .1 == *output || gate.0 .0 == *output {
            inputs_to_check.push(gate.2.clone());
        }
    }

    while inputs_to_check.len() > 0 {
        let next_input = inputs_to_check.pop().unwrap();

        for gate in gates {
            if gate.0 .1 == next_input || gate.0 .0 == next_input {
                // loop detected
                if gate.2 == *output {
                    return true;
                }
                // cont as usual
                if !seen_outputs.contains(&gate.2) {
                    inputs_to_check.push(gate.2.clone());
                    seen_outputs.push(gate.2.clone());
                }
            }
        }
    }
    false
}

fn main() {
    let input: String = read_file();
    let now = Instant::now();

    // part one
    let (states, gates) = parse_input(&input);
    let new_states = run(&states, &gates);

    let result = calc_result(&new_states, "z");
    println!("Result: {}", result);
    // part two

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_content_1() -> (String, usize) {
        (
            "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"
                .to_string(),
            4,
        )
    }

    fn get_test_content_2() -> (String, usize) {
        (
            "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"
                .to_string(),
            2024,
        )
    }

    fn get_test_content_3() -> (String, String) {
        (
            "x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00"
                .to_string(),
            "z00,z01,z02,z05".to_string(),
        )
    }

    #[test]
    fn test_solution_1_1() {
        let (input, solution) = get_test_content_1();
        let (states, gates) = parse_input(&input);
        let new_states = run(&states, &gates);

        let result = calc_result(&new_states, "z");
        assert_eq!(result, solution)
    }

    #[test]
    fn test_solution_1_2() {
        let (input, solution) = get_test_content_2();
        let (states, gates) = parse_input(&input);
        let new_states = run(&states, &gates);

        let result = calc_result(&new_states, "z");
        assert_eq!(result, solution)
    }

    #[test]
    fn test_solution_2() {
        let (input, solution) = get_test_content_3();
        let (states, gates) = parse_input(&input);
        let new_states = run(&states, &gates);

        let xresult = calc_result(&new_states, "x");
        let yresult = calc_result(&new_states, "y");
        let zresult = calc_result(&new_states, "z");
        println!("Resulting numbers: {} + {} = {}", xresult, yresult, zresult);
    }
}
