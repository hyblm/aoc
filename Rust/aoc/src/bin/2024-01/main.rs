fn main() {
    let (left, right) = parse_input();
    println!("part 1: {}", part1(&left, &right));
    println!("part 2: {}", part2(&left, &right));
}

fn parse_input() -> (Vec<usize>, Vec<usize>) {
    let input = include_str!("input.txt");

    let numbers = input.lines().map(|line| {
        let nums = line.split_once("   ").unwrap();
        let left: usize = nums.0.parse().unwrap();
        let right: usize = nums.1.parse().unwrap();
        (left, right)
    });
    let (mut list_1, mut list_2): (Vec<usize>, Vec<usize>) = numbers.unzip();

    list_1.sort_unstable();
    list_2.sort_unstable();
    (list_1, list_2)
}

fn part1(left: &[usize], right: &[usize]) -> usize {
    left.iter()
        .zip(right)
        .map(|(left, right)| left.abs_diff(*right))
        .sum()
}

fn part2(left: &[usize], right: &[usize]) -> usize {
    left.iter()
        .map(|x| x * right.iter().filter(|&n| n == x).count())
        .sum()
}
