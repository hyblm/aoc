static INPUT: &str = include_str!("input.txt");
fn main() {
    let (start_x, width, splitters) = parse_input(INPUT);
    println!("part 1: {}", part1(start_x, width, &splitters));
    println!("part 2: {}", part2(start_x, width, &splitters));
}

fn parse_input(input: &str) -> (usize, usize, Vec<usize>) {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let width = first_line.len();
    let start_x = first_line.find('S').unwrap();
    let splitters = lines
        .flat_map(|l| {
            l.bytes()
                .enumerate()
                .filter_map(move |(x, c)| (c == b'^').then_some(x))
        })
        .collect();
    (start_x, width, splitters)
}

fn part1(start_x: usize, width: usize, splitters: &[usize]) -> usize {
    let mut split_count = 0;
    let mut timeline_tails = vec![0; width];
    timeline_tails[start_x] = 1;
    for &x in splitters {
        if timeline_tails[x] == 1 {
            split_count += 1;
            timeline_tails[x] = 0;
            timeline_tails[x - 1] = 1;
            timeline_tails[x + 1] = 1;
        }
    }
    split_count
}

fn part2(start_x: usize, width: usize, splitters: &[usize]) -> usize {
    let mut timeline_tails = vec![0; width];
    timeline_tails[start_x] = 1;
    for &x in splitters {
        let beams = timeline_tails[x];
        timeline_tails[x] = 0;
        timeline_tails[x - 1] += beams;
        timeline_tails[x + 1] += beams;
    }
    timeline_tails.iter().sum()
}
