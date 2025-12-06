static INPUT: &str = include_str!("test.txt");
fn main() {
    let input = parse_input(INPUT);
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

type Input = ();

fn parse_input(input: &str) -> Input {
    input.lines().map(|line| ());
}

fn part1(banks: &Input) -> usize {
    // banks.iter().map(|bank| find_max_joltage(2, bank)).sum()
    0
}

fn part2(banks: &Input) -> usize {
    // banks.iter().map(|bank| find_max_joltage(12, bank)).sum()
    0
}
