use core::panic;

fn main() {
    let input = parse_input(include_str!("input.txt"));
    let part_one = part_one(&input);
    dbg!(part_one);
    let part_two = part_two(&input);
    dbg!(part_two);
}

fn part_one(input: &[(Vec<u32>, Vec<u32>)]) -> usize {
    input
        .iter()
        .map(|(winning, numbers)| {
            let match_count = numbers.iter().filter(|x| winning.contains(x)).count() as u32;
            if match_count == 0 {
                0
            } else {
                2_i32.pow(match_count - 1) as usize
            }
        })
        .sum()
}

fn part_two(input: &[(Vec<u32>, Vec<u32>)]) -> usize {
    input
        .iter()
        .scan(
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            |multipliers: &mut [usize; 11], (winning, numbers)| {
                let match_count = numbers.iter().filter(|x| winning.contains(x)).count();
                let card_count = multipliers[0];
                for i in 0..multipliers.len() {
                    multipliers[i] = multipliers.get(i + 1).unwrap_or(&1)
                        + if i < match_count { card_count } else { 0 };
                }
                Some(card_count)
            },
        )
        .sum()
}

fn parse_input(input: &str) -> Vec<(Vec<u32>, Vec<u32>)> {
    input
        .lines()
        .map(|l| {
            let Some((_, l)) = l.split_once(":") else {
                panic!("No ':' in line")
            };
            let Some((win_str, num_str)) = l.split_once("|") else {
                panic!("No '|' in line")
            };
            let winning = win_str
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();

            let numbers = num_str
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            (winning, numbers)
        })
        .collect()
}
