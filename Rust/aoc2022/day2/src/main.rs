fn main() {
    let input: String = include_str!("input.txt").split_whitespace().collect();

    let part_one: usize = input
        .as_bytes()
        .chunks(2)
        .map(|x| (usize::from(b'D' - x[0]), usize::from(x[1] - b'W')))
        .map(|(op, me)| {
            let result = (op + me) % 3;
            result * 3 + me
        })
        .sum();

    println!("Part One: {}", part_one);

    let part_two: usize = input
        .as_bytes()
        .chunks(2)
        .map(|x| (usize::from(x[0] - b'A'), usize::from(x[1] - b'X')))
        .map(|(op, result)| {
            let shape = (op + result + 2) % 3;
            result * 3 + shape + 1
        })
        .sum();

    println!("Part two: {}", part_two);
}
