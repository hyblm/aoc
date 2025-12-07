static INPUT: &str = include_str!("input.txt");
fn main() {
    let input = parse_input(INPUT);
    // dbg!(&input);
    // println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

#[derive(Debug, Clone, Copy)]
struct Position {
    y: usize,
    x: usize,
}
#[derive(Debug)]
struct Input {
    start_x: usize,
    splitters: Vec<Position>,
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();
    let start_x = lines.next().unwrap().find('S').unwrap();
    let splitters = lines
        .enumerate()
        .flat_map(move |(y, l)| {
            l.bytes()
                .enumerate()
                .filter_map(move |(x, c)| (c == b'^').then_some(Position { y, x }))
        })
        .collect();
    Input { start_x, splitters }
}

fn part1(input: &Input) -> usize {
    let mut split_count = 0;
    let Input { start_x, splitters } = input;
    let mut beams = Vec::with_capacity(splitters.len() * 2);
    beams.push(*start_x);
    let mut current_y = 1;
    for Position { y, x } in splitters {
        if *y > current_y {
            beams.sort();
            beams.dedup();
            current_y = *y;
        }
        if let Some(index) = beams.iter().position(|i| i == x) {
            split_count += 1;
            beams[index] -= 1;
            beams.push(x + 1);
        }
    }
    split_count
}

fn part2_old(input: &Input) -> usize {
    let Input { start_x, splitters } = input;
    let mut timeline_tails = Vec::with_capacity(splitters.len() * 2);
    let mut indexes = Vec::new();
    timeline_tails.push(*start_x);
    let mut current_y = 1;
    for Position { y, x } in splitters {
        if *y > current_y {
            timeline_tails.sort();
            current_y = *y;
            // println!("{current_y}: {beams:?}");
        }
        if let Some(start) = timeline_tails.iter().position(|i| i == x) {
            indexes.extend(
                timeline_tails[start..]
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, i)| (i == x).then_some(idx)),
            );
            for &index in &indexes {
                timeline_tails[index] -= 1;
                timeline_tails.push(x + 1);
            }
            indexes.clear();
        }
    }
    // println!("{current_y}: {beams:?}");
    timeline_tails.len()
}

fn part2(input: &Input) -> usize {
    let Input { start_x, splitters } = input;
    let mut timeline_tails = [0; 150];
    timeline_tails[*start_x] = 1;
    let mut current_y = 1;
    for &Position { y, x } in splitters {
        let beams = timeline_tails[x];
        timeline_tails[x] = 0;
        timeline_tails[x - 1] += beams;
        timeline_tails[x + 1] += beams;
    }
    timeline_tails.iter().sum()
}
