fn main() {
    // part1();
    part2();
}

fn part1() {
    let input = include_str!("test.txt");

    let numbers: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut list_1 = Vec::with_capacity(numbers.len() / 2);
    let mut list_2 = Vec::with_capacity(numbers.len() / 2);

    let mut chunks_exact = numbers.chunks_exact(2);
    while let Some(&[num1, num2]) = chunks_exact.next() {
        list_1.push(num1);
        list_2.push(num2);
    }

    list_1.sort_unstable();
    list_2.sort_unstable();

    let part1: usize = list_1
        .iter()
        .zip(&list_2)
        .map(|(left, right)| left.abs_diff(*right))
        .sum();

    println!("{part1}");
}

fn part2() {
    let input = include_str!("input.txt");

    let numbers: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut list_1 = Vec::with_capacity(numbers.len() / 2);
    let mut list_2 = Vec::with_capacity(numbers.len() / 2);

    let mut chunks_exact = numbers.chunks_exact(2);
    while let Some(&[num1, num2]) = chunks_exact.next() {
        list_1.push(num1);
        list_2.push(num2);
    }

    list_1.sort_unstable();
    list_2.sort_unstable();

    let result: usize = list_1
        .iter()
        .map(|x| x * list_2.iter().filter(|&n| n == x).count())
        .sum();

    println!("{result}");
}
