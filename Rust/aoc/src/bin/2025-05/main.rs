use std::ops::Range;
static INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);
    println!("part 1: {}", part1(&input.0, &input.1));
    println!("part 2: {}", part2(input.0));
}

fn parse_input(input: &str) -> (Vec<Range<usize>>, Vec<usize>) {
    let (ranges, ids) = input.split_once("\n\n").unwrap();
    let ranges = ranges
        .lines()
        .map(|line| {
            let (x, y) = line.split_once('-').unwrap();
            x.parse().unwrap()..y.parse::<usize>().unwrap() + 1
        })
        .collect();
    let ids = ids.lines().map(|x| x.parse().unwrap()).collect();

    (ranges, ids)
}

fn part1(ranges: &[Range<usize>], ids: &[usize]) -> usize {
    ids.iter()
        .filter(|id| ranges.iter().any(|r| r.contains(id)))
        .count()
}

fn part2(mut ranges: Vec<Range<usize>>) -> usize {
    aoc::ranges_merge_overlapping(&mut ranges)
        .iter()
        .map(|r| r.end - r.start)
        .sum()
}
