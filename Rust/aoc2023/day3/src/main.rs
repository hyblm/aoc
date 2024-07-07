fn main() {
    let input = include_str!("input.txt");
    // let input = parse_input(input);
    part_one(input);
    part_two(input);
}

fn part_one(schematic: &str) {
    let parts: Vec<_> = schematic
        .char_indices()
        .filter(|&(_, x)| x.is_ascii_punctuation() && x != '.')
        .collect();

    let line_len = schematic.chars().position(|c| c == '\n').unwrap() + 1;
    let nums = numbers_indices(schematic, line_len);

    let x: u32 = nums
        .iter()
        .filter_map(|(range, num)| {
            if parts.iter().any(|&(i, _)| in_bounds(range, i, line_len)) {
                return Some(*num);
            };
            None
        })
        .sum();

    println!("Part One: {}", x);
}

fn part_two(schematic: &str) {
    let gears = schematic
        .char_indices()
        .filter_map(|(i, x)| if x == '*' { Some(i) } else { None });

    let line_len = schematic.chars().position(|c| c == '\n').unwrap() + 1;
    let nums = numbers_indices(schematic, line_len);

    let x: u32 = gears
        .filter_map(|i| {
            let gear_edges = nums.iter().filter_map(|(range, num)| {
                if in_bounds(range, i, line_len) {
                    Some(num)
                } else {
                    None
                }
            });
            if gear_edges.clone().count() > 1 {
                Some(gear_edges.product::<u32>())
            } else {
                None
            }
        })
        .sum();
    println!("Part Two: {:?}", x);
}

fn in_bounds(range: &std::ops::Range<usize>, i: usize, line_len: usize) -> bool {
    range.contains(&i) || range.contains(&(i + line_len)) || range.contains(&(i - line_len))
}

fn numbers_indices(schematic: &str, line_len: usize) -> Vec<(std::ops::Range<usize>, u32)> {
    let mut nums = Vec::new();
    let mut search = schematic;
    let mut index = 0;

    while let Some(start) = search.find(char::is_numeric) {
        (_, search) = search.split_at(start);

        let end = search
            .find(|c: char| !c.is_numeric())
            .unwrap_or(schematic.len());

        let num;
        (num, search) = search.split_at(end);

        index += start;
        let start = if index > 0 { index - 1 } else { 0 };
        index += end;
        let end = if index > 0 {
            index.next_multiple_of(line_len).min(index + 1)
        } else {
            index + 1
        };

        nums.push((start..end, num.parse::<u32>().unwrap()));
    }
    nums
}
