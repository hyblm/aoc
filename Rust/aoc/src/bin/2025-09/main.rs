static INPUT: &str = include_str!("input.txt");
fn main() {
    let red_tile_positions = parse_input(INPUT);
    println!("part 1: {}", part1(&red_tile_positions));
    println!("part 2: {}", part2(red_tile_positions));
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

fn part1(positions: &[(usize, usize)]) -> usize {
    let rectangle_areas = aoc::all_pairs(positions).map(|(p1, p2)| area(p1, p2));
    rectangle_areas.max().unwrap()
}

fn part2(positions: Vec<(usize, usize)>) -> usize {
    // To also check the line between the last and first point in the iterator
    let mut checks = positions.clone();
    checks.push(checks[0]);

    let rectangle_areas = aoc::all_pairs(&positions)
        .filter(|(p1, p2)| no_line_intersects_rect(&checks, p1, p2))
        .map(|(p1, p2)| area(p1, p2));
    rectangle_areas.max().unwrap()
}

#[rustfmt::skip]
fn no_line_intersects_rect(
    checks: &[(usize, usize)],
    p1: &(usize, usize),
    p2: &(usize, usize),
) -> bool {
    let (y_max, y_min) = if p1.1 > p2.1 { (p1.1, p2.1) } else { (p2.1, p1.1) };
    let (x_max, x_min) = if p1.0 > p2.0 { (p1.0, p2.0) } else { (p2.0, p1.0) };

    checks.windows(2).all(|p| {
        let both_left  = p[0].0 <= x_min && p[1].0 <= x_min;
        let both_right = p[0].0 >= x_max && p[1].0 >= x_max;
        let both_above = p[0].1 <= y_min && p[1].1 <= y_min;
        let both_below = p[0].1 >= y_max && p[1].1 >= y_max;
        (both_left || both_right) || (both_above || both_below)
    })
}

fn area(p1: (usize, usize), p2: (usize, usize)) -> usize {
    let a = p1.0.abs_diff(p2.0) + 1;
    let b = p1.1.abs_diff(p2.1) + 1;
    a * b
}
