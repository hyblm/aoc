static INPUT: &str = include_str!("input.txt");
fn main() {
    let input = parse_input(INPUT);
    println!("part 1: {}", part1(&input.0, &input.1));
    println!("part 2: {}", part2(input.0));
}

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let (ranges, ids) = input.split_once("\n\n").unwrap();
    let ranges = ranges
        .lines()
        .map(|line| {
            let (x, y) = line.split_once('-').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();
    let ids = ids.lines().map(|x| x.parse().unwrap()).collect();

    (ranges, ids)
}

fn part1(ranges: &[(usize, usize)], ids: &[usize]) -> usize {
    ids.iter()
        .filter(|id| ranges.iter().any(|(x, y)| (x..=y).contains(id)))
        .count()
}

fn part2(mut ranges: Vec<(usize, usize)>) -> usize {
    let mut fresh_ids = 0;
    ranges.sort();
    let (mut start, mut end) = ranges[0];
    for range in &ranges[1..] {
        if range.0 <= end + 1 {
            if range.1 > end {
                end = range.1;
            }
        } else {
            fresh_ids += end + 1 - start;
            start = range.0;
            end = range.1;
        }
    }
    fresh_ids + end + 1 - start
}
