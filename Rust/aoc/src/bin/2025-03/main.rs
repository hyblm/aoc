static INPUT: &str = include_str!("input.txt");
fn main() {
    let input = parse_input(INPUT);
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect())
        .collect()
}

fn part1(banks: &[Vec<u8>]) -> usize {
    banks.iter().map(|bank| find_max_joltage(2, bank)).sum()
}

fn part2(banks: &[Vec<u8>]) -> usize {
    banks.iter().map(|bank| find_max_joltage(12, bank)).sum()
}

fn find_max_joltage(battery_count: u32, bank: &[u8]) -> usize {
    (0..battery_count)
        .rev()
        .scan(bank, |bank, exp| {
            let result = bank[..bank.len() - exp as usize]
                .iter()
                .enumerate()
                .rev()
                .max_by_key(|x| x.1)
                .unwrap();
            (_, *bank) = bank.split_at(result.0 + 1);
            Some(usize::from(*result.1) * 10usize.pow(exp))
        })
        .sum()
}
