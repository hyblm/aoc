fn main() {
    let nums: Vec<_> = include_str!("input.txt")
        .lines()
        .flat_map(|line| {
            line.split(|c| c == '-' || c == ',')
                .flat_map(|x| x.parse::<usize>())
        })
        .collect();

    let part_one = nums
        .chunks_exact(4)
        .filter(|x| (x[0] <= x[2]) && (x[1] >= x[3]) || (x[0] >= x[2]) && (x[1] <= x[3]))
        .count();
    dbg!(part_one);

    let part_two = nums
        .chunks_exact(4)
        .filter(|x| (x[0] <= x[3]) && (x[2] <= x[1]))
        .count();
    dbg!(part_two);
}
