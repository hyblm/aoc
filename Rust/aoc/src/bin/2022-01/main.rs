fn main() {
    let mut elves: Vec<_> = include_str!("input.txt")
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|l| l.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect();
    elves.sort_unstable();

    let part_one = elves.iter().last().unwrap();
    println!("Part One: {part_one}");

    let part_two: usize = elves.iter().rev().take(3).sum();
    println!("Part Two: {part_two:?}");
}
