static INPUT: &str = include_str!("input.txt");
fn main() {
    let input = parse_input(INPUT);
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(input));
}

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect()
}

fn part1(map: &[Vec<bool>]) -> usize {
    (0..map.len())
        .flat_map(|y| {
            (0..map[0].len()).map(move |x| {
                let accessible = map[y][x] && adjacent_count(map, y, x) < 4;
                if accessible { 1 } else { 0 }
            })
        })
        .sum()
}

fn part2(mut map: Vec<Vec<bool>>) -> usize {
    let mut remove_count = 0;
    loop {
        let mut rolls_removed_in_pass = 0;
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if map[y][x] && adjacent_count(&map, y, x) < 4 {
                    map[y][x] = false;
                    rolls_removed_in_pass += 1;
                }
            }
        }
        remove_count += rolls_removed_in_pass;
        if rolls_removed_in_pass == 0 {
            break remove_count;
        }
    }
}

fn adjacent_count(map: &[Vec<bool>], y: usize, x: usize) -> i32 {
    let width = map[0].len();
    let height = map.len();
    let mut adjacent_count = -1;
    for y_offset in -1..=1 {
        for x_offset in -1..=1 {
            let new_y = (y as isize + y_offset) as usize;
            if new_y >= height {
                continue;
            }
            let new_x = (x as isize + x_offset) as usize;
            if new_x >= width {
                continue;
            }
            if map[new_y][new_x] {
                adjacent_count += 1;
            }
        }
    }
    adjacent_count
}
