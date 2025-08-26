use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    let (instructions, nodes) = input.split_once("\n\n").unwrap();
    let network = parse_network(nodes);
    let step_count = walk_network_parallel(instructions, &network);

    println!("{step_count}")
}

fn walk_network_parallel(instructions: &str, network: &HashMap<NodeId, (NodeId, NodeId)>) -> usize {
    let mut instructions = instructions.as_bytes().iter().cycle();

    let mut step_count = 0;

    let is_start_node = |id: NodeId| id.as_bytes()[2] == b'A';
    let mut current: Vec<NodeId> = network
        .keys()
        .copied()
        .filter(|&k| is_start_node(k))
        .collect();
    let mut ends = Vec::with_capacity(current.len());

    while ends.len() != current.len() {
        let direction = *instructions.next().unwrap();
        for node in &mut current {
            let is_end = node.as_bytes()[2] == b'Z';
            if is_end {
                ends.push(step_count);
            }
            *node = match direction {
                b'L' => network.get(node).expect("key exists").0,
                b'R' => network.get(node).expect("key exists").1,
                _ => unreachable!(),
            }
        }
        step_count += 1;
    }
    let mut steps_necessary = ends[0];
    for step_count in ends.into_iter().skip(1) {
        steps_necessary = lcm(steps_necessary, step_count);
    }

    steps_necessary
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while a % b > 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    b
}

#[allow(unused)]
fn walk_network(instructions: &str, network: &HashMap<NodeId, (NodeId, NodeId)>) -> usize {
    let mut instructions = instructions.as_bytes().iter().cycle();

    let mut step_count = 0;

    let end = "ZZZ";
    let mut current = "AAA";

    while current != end {
        let direction = *instructions.next().unwrap();
        current = match direction {
            b'L' => network.get(&current).expect("key exists").0,
            b'R' => network.get(&current).expect("key exists").1,
            _ => unreachable!(),
        };
        step_count += 1;
    }

    step_count
}

type NodeId = &'static str;
fn parse_network(nodes: &'static str) -> HashMap<NodeId, (NodeId, NodeId)> {
    HashMap::from_iter(nodes.lines().map(|line| {
        let (node_id, rest) = line.split_once(" = (").unwrap();
        let (left, rest) = rest.split_once(", ").unwrap();
        let (right, _) = rest.split_once(')').unwrap();

        (node_id, (left, right))
    }))
}

#[cfg(test)]
mod tests {
    use crate::{parse_network, walk_network, walk_network_parallel};

    #[test]
    fn example1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let (instructions, nodes) = input.split_once("\n\n").unwrap();
        let network = parse_network(nodes);
        let step_count = walk_network(instructions, &network);
        assert_eq!(step_count, 2);
    }

    #[test]
    fn example2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let (instructions, nodes) = input.split_once("\n\n").unwrap();
        let network = parse_network(nodes);
        let step_count = walk_network(instructions, &network);
        assert_eq!(step_count, 6);
    }

    #[test]
    fn parallel() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let (instructions, nodes) = input.split_once("\n\n").unwrap();
        let network = parse_network(nodes);
        let step_count = walk_network_parallel(instructions, &network);
        assert_eq!(step_count, 6);
    }
}
