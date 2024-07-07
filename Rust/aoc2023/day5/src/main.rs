use std::ops::{ControlFlow, Range};
use std::str::FromStr;

fn main() {
    let (seeds, input) = include_str!("input.txt").split_once("\n\n").unwrap();
    let seeds = str_to_vec(seeds.strip_prefix("seeds: ").unwrap());
    let mappings = parse_input(input);

    let part_one = part_one(&seeds, &mappings);
    println!("Part one: {}", part_one);

    let seeds: Vec<Range<isize>> = seeds
        .chunks(2)
        .map(|r| {
            let start = r[0];
            start..(start + r[1])
        })
        .collect();

    let part_two = part_two(&seeds, &mappings);
    println!("Part two: {}", part_two);
}

#[derive(Debug, Clone)]
struct Mapping {
    range: Range<isize>,
    delta: isize,
}

fn part_one(seeds: &[isize], steps: &[Vec<Mapping>]) -> isize {
    seeds
        .iter()
        .map(|seed| {
            steps.iter().fold(*seed, |seed, mappings| {
                match mappings.iter().find(|&x| x.range.contains(&seed)) {
                    Some(mapping) => seed + mapping.delta,
                    None => seed,
                }
            })
        })
        .min()
        .expect("seeds is not empty")
}

fn part_two(seeds: &[Range<isize>], steps: &[Vec<Mapping>]) -> isize {
    steps
        .into_iter()
        .fold(Vec::from(seeds), |mut ranges, mappings| {
            ranges.iter_mut().fold(vec![], |mut results, range| {
                for map in mappings {
                    if let ControlFlow::Break(_) = map_range(range, map, &mut results) {
                        return results;
                    }
                }
                results.push(range.clone());
                results
            })
        })
        .iter()
        .map(|range| range.start)
        .min()
        .expect("is not empty") as isize
}

fn map_range(
    range: &mut Range<isize>,
    map: &Mapping,
    results: &mut Vec<Range<isize>>,
) -> ControlFlow<()> {
    let overlap = range.start.max(map.range.start)..range.end.min(map.range.end);

    if !overlap.is_empty() {
        let mapped = (overlap.start + map.delta)..(overlap.end + map.delta);
        results.push(mapped);

        let left = range.start..overlap.start;
        if !left.is_empty() {
            results.push(left);
        }

        let right = overlap.end..range.end;
        if !right.is_empty() {
            *range = right;
        } else {
            return ControlFlow::Break(());
        }
    }
    ControlFlow::Continue(())
}

fn parse_input(input: &'static str) -> Vec<Vec<Mapping>> {
    input
        .split("\n\n")
        .map(|map| {
            let lines = map.lines().skip(1);
            let mut res: Vec<_> = lines
                .map(|mapping| {
                    let mapping = str_to_vec(mapping);
                    let start = mapping[1];
                    let end = start + mapping[2];
                    let delta = mapping[0] as isize - start as isize;
                    Mapping {
                        range: start..end,
                        delta,
                    }
                })
                .collect();
            res.sort_by_key(|x| x.range.start);
            res
        })
        .collect()
}

fn str_to_vec<T: FromStr>(line: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    line.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}
