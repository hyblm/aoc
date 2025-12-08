use std::collections::HashMap;

static INPUT: &str = include_str!("input.txt");
fn main() {
    let input = parse_input(INPUT);
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn parse_input(input: &str) -> Vec<(usize, usize, usize)> {
    input
        .lines()
        .map(|line| {
            let (x, rest) = line.split_once(',').unwrap();
            let (y, z) = rest.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap())
        })
        .collect()
}

fn part1(junction_boxes: &[(usize, usize, usize)]) -> usize {
    let mut distances: Vec<_> = junction_boxes
        .iter()
        .enumerate()
        .flat_map(|(i, (x, y, z))| {
            junction_boxes[i + 1..]
                .iter()
                .enumerate()
                .map(move |(j, &(x2, y2, z2))| {
                    let distance = (x - x2).pow(2) + (y - y2).pow(2) + (z - z2).pow(2);
                    (i, j + i + 1, distance)
                })
        })
        .collect();
    distances.sort_by_key(|d| d.2);
    let mut circuit_map: HashMap<usize, usize> = HashMap::new();
    let mut circuits: Vec<Vec<usize>> = Vec::new();
    for &(i, j, _) in &distances[..1000] {
        match (circuit_map.get(&i), circuit_map.get(&j)) {
            (None, None) => {
                circuit_map.insert(i, circuits.len());
                circuit_map.insert(j, circuits.len());
                circuits.push(vec![i, j]);
            }
            (None, Some(&idx)) => {
                circuit_map.insert(i, idx);
                circuits[idx].push(i);
            }
            (Some(&idx), None) => {
                circuit_map.insert(j, idx);
                circuits[idx].push(j);
            }
            (Some(a), Some(b)) if a == b => (),
            (Some(&a), Some(&b)) => {
                let [circuit_a, circuit_b] = circuits.get_disjoint_mut([a, b]).unwrap();
                for c in circuit_b.drain(..) {
                    *circuit_map.get_mut(&c).unwrap() = a;
                    circuit_a.push(c);
                }
            }
        };
    }
    circuits.sort_unstable_by_key(|c| c.len());
    circuits.iter().rev().take(3).map(|c| c.len()).product()
}

fn part2(junction_boxes: &[(usize, usize, usize)]) -> usize {
    let mut distances: Vec<_> = junction_boxes
        .iter()
        .enumerate()
        .flat_map(|(i, (x, y, z))| {
            junction_boxes[i + 1..]
                .iter()
                .enumerate()
                .map(move |(j, &(x2, y2, z2))| {
                    let distance = (x - x2).pow(2) + (y - y2).pow(2) + (z - z2).pow(2);
                    (i, j + i + 1, distance)
                })
        })
        .collect();
    distances.sort_by_key(|d| d.2);
    let mut circuit_map: HashMap<usize, usize> = HashMap::new();
    let mut circuits: Vec<Vec<usize>> = Vec::new();
    for (i, j, _) in distances {
        let idx = match (circuit_map.get(&i), circuit_map.get(&j)) {
            (None, None) => {
                let idx = circuits.len();
                circuit_map.insert(i, idx);
                circuit_map.insert(j, idx);
                circuits.push(vec![i, j]);
                idx
            }
            (None, Some(&idx)) => {
                circuit_map.insert(i, idx);
                circuits[idx].push(i);
                idx
            }
            (Some(&idx), None) => {
                circuit_map.insert(j, idx);
                circuits[idx].push(j);
                idx
            }
            (Some(&a), Some(&b)) if a == b => a,
            (Some(&a), Some(&b)) => {
                let [circuit_a, circuit_b] = circuits.get_disjoint_mut([a, b]).unwrap();
                for c in circuit_b.drain(..) {
                    *circuit_map.get_mut(&c).unwrap() = a;
                    circuit_a.push(c);
                }
                a
            }
        };
        if circuits[idx].len() == junction_boxes.len() {
            return junction_boxes[i].0 * junction_boxes[j].0;
        }
    }
    0
}
