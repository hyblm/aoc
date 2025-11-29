fn main() {
    let part_one: usize = include_str!("input.txt")
        .lines()
        .map(|line| {
            let mid = line.len() / 2;
            let (comp1, comp2) = line.split_at(mid);

            let item = comp1
                .bytes()
                .find_map(|c| comp2.bytes().find(|&b| b == c))
                .expect("There is always exactly one duplicate item");

            usize::from(priority(item))
        })
        .sum();

    println!("Part One: {}", part_one);

    let rucksacks: Vec<_> = include_str!("input.txt").lines().collect();
    let part_two: usize = rucksacks
        .chunks_exact(3)
        .map(|team| {
            let [a, b, c] = team.try_into().ok().unwrap();
            let item: u8 = a
                .bytes()
                .find_map(|a| {
                    b.bytes()
                        .find_map(|b| c.bytes().find(|&c| c == b && c == a))
                })
                .expect("exactly 1 item is present in all three");
            usize::from(priority(item))
        })
        .sum();

    println!("Part two: {}", part_two);
}

fn priority(item: u8) -> u8 {
    if item.is_ascii_lowercase() {
        item - (b'a' - 1)
    } else {
        item - (b'A' - 1) + 26
    }
}
