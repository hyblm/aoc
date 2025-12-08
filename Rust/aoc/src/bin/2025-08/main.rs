use std::collections::HashMap;

static INPUT: &str = include_str!("input.txt");
#[allow(arithmetic_overflow)]
fn main() {
    let junction_boxes = parse_input(INPUT);
    let mut distances: Vec<_> = junction_boxes
        .iter()
        .enumerate()
        .flat_map(|(i, &(x, y, z))| {
            junction_boxes[i + 1..]
                .iter()
                .enumerate()
                .map(move |(j, &(x2, y2, z2))| (i, j + i + 1, distance(x, y, z, x2, y2, z2)))
        })
        .collect();
    distances.sort_by_key(|d| d.2);
    println!("part 1: {}", part1(&distances));
    println!("part 2: {}", part2(&junction_boxes, &distances));
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

fn part1(distances: &[(usize, usize, usize)]) -> usize {
    let mut circuit_map: HashMap<usize, usize> = HashMap::new();
    let mut circuits: Vec<Vec<usize>> = Vec::new();
    for &(i, j, _) in &distances[..1000] {
        make_connection(&mut circuit_map, &mut circuits, i, j);
    }
    circuits.sort_unstable_by_key(|c| c.len());
    circuits.iter().rev().take(3).map(|c| c.len()).product()
}

fn part2(junction_boxes: &[(usize, usize, usize)], distances: &[(usize, usize, usize)]) -> usize {
    let mut circuit_map: HashMap<usize, usize> = HashMap::new();
    let mut circuits: Vec<Vec<usize>> = Vec::new();
    for &(i, j, _) in distances {
        let idx = make_connection(&mut circuit_map, &mut circuits, i, j);
        if circuits[idx].len() == junction_boxes.len() {
            return junction_boxes[i].0 * junction_boxes[j].0;
        }
    }
    0
}

/// Returns the index of the `circuit` that boxes `i` and `j` are now connected in
fn make_connection(
    circuit_map: &mut HashMap<usize, usize>,
    circuits: &mut Vec<Vec<usize>>,
    i: usize,
    j: usize,
) -> usize {
    match (circuit_map.get(&i), circuit_map.get(&j)) {
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
            for box_idx in circuit_b.drain(..) {
                *circuit_map.get_mut(&box_idx).unwrap() = a;
                circuit_a.push(box_idx);
            }
            a
        }
    }
}

fn distance(x: usize, y: usize, z: usize, x2: usize, y2: usize, z2: usize) -> usize {
    (x.wrapping_sub(x2)).wrapping_pow(2)
        + (y.wrapping_sub(y2)).wrapping_pow(2)
        + (z.wrapping_sub(z2)).wrapping_pow(2)
}
