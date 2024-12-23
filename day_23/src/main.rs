use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

fn read_file() -> String {
    let content = fs::read_to_string("./input").expect("Something went wrong reading the file");
    return content;
}

fn parse_input(input: &String) -> HashMap<&str, HashSet<&str>> {
    let mut computers: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in input.lines() {
        let conn: Vec<&str> = line.split("-").collect();
        assert_eq!(conn.len(), 2);
        computers.entry(conn[0]).or_default().insert(conn[1]);
        computers.entry(conn[1]).or_default().insert(conn[0]);
    }
    computers
}

fn find_loops_by_distance<'a>(
    graph: &'a HashMap<&'a str, HashSet<&'a str>>,
    start: &'a str,
    current: &'a str,
    seen: &mut HashSet<&'a str>,
    distance: usize,
) -> Vec<HashSet<&'a str>> {
    seen.insert(current);

    // we've gone too far
    if seen.len() > distance {
        return Vec::new();
    }

    // we have our loop
    let connected = graph.get(current).unwrap();
    if seen.len() == distance && connected.contains(start) {
        return vec![seen.clone()];
    }

    let mut loops: Vec<HashSet<&str>> = Vec::new();
    // lets look further
    for comp in connected {
        // we don't want loops in loops
        if seen.contains(comp) {
            continue;
        }

        // lets look further
        // Note that this will add duplicates into the list that have to be filtered later on
        loops.extend(find_loops_by_distance(
            graph,
            start,
            comp,
            &mut seen.clone(),
            distance,
        ));
    }

    loops
}

// solution for part one does not work nicely, so lets change it a little
fn find_largest_set<'a>(graph: HashMap<&'a str, HashSet<&'a str>>) -> Vec<&'a str> {
    let computers = graph.clone();

    let mut largest_comb: Vec<&str> = Vec::new();
    'comp_loop: for (comp, conns) in computers.iter() {
        for possible_len in (0..conns.len()).rev() {
            'comb_loop: for combination in conns.iter().combinations(possible_len) {
                for elem in combination.iter() {
                    let mut temp_set: HashSet<&str> =
                        HashSet::from_iter(combination.iter().map(|&&x| x)); // dont know why i had to map, i asked copilot here
                    temp_set.remove(*elem);
                    temp_set.insert(comp);
                    if !temp_set.is_subset(graph.get(*elem).unwrap()) {
                        continue 'comb_loop;
                    }
                }
                // found largest solution for start computer
                if combination.len() + 1 > largest_comb.len() {
                    largest_comb = combination.into_iter().map(|&x| x).collect();
                    largest_comb.push(comp);
                    // dont know why i had to map, i asked copilot here
                }
                continue 'comp_loop;
            }
        }
    }
    largest_comb
}

fn main() {
    let input: String = read_file();
    let now = Instant::now();

    // part one
    let computers = parse_input(&input);

    let mut all_loops: Vec<HashSet<&str>> = Vec::new();
    for (comp, _) in computers.iter() {
        if !comp.starts_with("t") {
            continue;
        }
        let mut seen: HashSet<&str> = HashSet::new();
        for l in find_loops_by_distance(&computers, &comp, &comp, &mut seen, 3) {
            if !all_loops.contains(&l) {
                all_loops.push(l);
            }
        }
    }
    println!(
        "Found {} loops that contain computer start with t",
        all_loops.len()
    );

    // part two
    let mut largest = find_largest_set(computers);
    largest.sort();

    println!(
        "Longest set of connections: {:?}, length: {:?}",
        largest.join(","),
        largest.len()
    );

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_content() -> String {
        "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"
            .to_string()
    }
    #[test]
    fn test_solution_1() {
        let input = get_test_content();
        let computers = parse_input(&input);

        let mut all_loops: Vec<HashSet<&str>> = Vec::new();
        for (comp, _) in computers.iter() {
            let mut seen: HashSet<&str> = HashSet::new();
            for l in find_loops_by_distance(&computers, &comp, &comp, &mut seen, 3) {
                if !all_loops.contains(&l) {
                    all_loops.push(l);
                }
            }
        }

        assert_eq!(all_loops.len(), 12)
    }

    #[test]
    fn test_solution_1_1() {
        let input = get_test_content();
        let computers = parse_input(&input);

        let mut all_loops: Vec<HashSet<&str>> = Vec::new();
        for (comp, _) in computers.iter() {
            if !comp.starts_with("t") {
                continue;
            }
            let mut seen: HashSet<&str> = HashSet::new();
            for l in find_loops_by_distance(&computers, &comp, &comp, &mut seen, 3) {
                if !all_loops.contains(&l) {
                    all_loops.push(l);
                }
            }
        }

        assert_eq!(all_loops.len(), 7)
    }

    #[test]
    fn test_solution_2() {
        let input = get_test_content();
        let computers = parse_input(&input);

        let largest = find_largest_set(computers);

        println!(
            "Longest set of connections: {:?}, length: {:?}",
            largest,
            largest.len()
        );
    }
}
