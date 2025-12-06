static INPUT: &str = include_str!("input.txt");
fn main() {
    let input = parse_input(INPUT);
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .trim()
        .split(',')
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            let start: usize = start.parse().unwrap();
            let end: usize = end.parse().unwrap();
            (start, end)
        })
        .collect()
}

fn part1(ranges: &[(usize, usize)]) -> usize {
    ranges
        .iter()
        .flat_map(|range| {
            (range.0..=range.1).filter_map(|id| validate_id_inner(id, count_digits(id), 2))
        })
        .sum()
}

fn part2(ranges: &[(usize, usize)]) -> usize {
    ranges
        .iter()
        .flat_map(|range| (range.0..=range.1).map(validate_id))
        .sum()
}

fn validate_id(id: usize) -> usize {
    let digit_count = count_digits(id);
    for repetition in 2..=digit_count {
        if digit_count.is_multiple_of(repetition)
            && let Some(invalid_id) = validate_id_inner(id, digit_count, repetition)
        {
            return invalid_id;
        };
    }
    0
}

fn validate_id_inner(id: usize, digit_count: u32, repetition_count: u32) -> Option<usize> {
    let exp = digit_count / repetition_count;
    let sentinel = {
        let mut sentinel = 1;
        for i in 1..repetition_count {
            sentinel += 10usize.pow(i * exp)
        }
        sentinel
    };
    if id.is_multiple_of(sentinel) {
        return Some(id);
    }
    None
}

fn count_digits(start: usize) -> u32 {
    let mut exponent = 0;
    let radix = 10;
    let mut divisor = 1;
    while (start / divisor) > 0 {
        exponent += 1;
        divisor *= radix;
    }
    exponent
}
