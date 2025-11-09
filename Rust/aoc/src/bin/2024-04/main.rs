static INPUT: &str = include_str!("input.txt");
const DIM: &str = "\x1b[30m";
const RESET: &str = "\x1b[0m";

fn main() {
    // part1();
    part2();
}

fn part1() {
    let mut result = 0;
    let pattern = "XMAS";
    let map: Vec<_> = INPUT.lines().flat_map(|x| x.chars()).collect();
    let stride = INPUT.find('\n').unwrap();

    // EAST
    result += fun_name(
        pattern,
        &map,
        |y, x| y * stride + x,
        0..stride,
        |_| 0..stride,
    );

    // WEST
    result += fun_name(
        pattern,
        &map,
        |y, x| y * stride + x,
        0..stride,
        |_| (0..stride).rev(),
    );

    // SOUTH
    result += fun_name(
        pattern,
        &map,
        |x, y| y * stride + x,
        0..stride,
        |_| 0..stride,
    );

    // NORTH
    result += fun_name(
        pattern,
        &map,
        |x, y| y * stride + x,
        0..stride,
        |_| (0..stride).rev(),
    );

    // SOUTH-WEST
    let diag = (stride - 1) * 2;
    let range = 0..=diag;
    let range_fn = |y| 0..(stride - ((diag / 2) as isize - y as isize).unsigned_abs());
    let index_fn = |y, x| y + ((1 + y % stride) * (y / stride) + x) * (stride - 1);
    result += fun_name(pattern, &map, index_fn, range, range_fn);

    // NORTH-EAST
    let diag = (stride - 1) * 2;
    let range = 0..=diag;
    let range_fn = |y| (0..(stride - ((diag / 2) as isize - y as isize).unsigned_abs())).rev();
    let index_fn = |y, x| y + ((1 + y % stride) * (y / stride) + x) * (stride - 1);
    result += fun_name(pattern, &map, index_fn, range, range_fn);

    // SOUTH-EAST
    let range = 0..=diag;
    let range_fn = |y| 0..(stride - (y % stride) - (y / stride));
    let index_fn = |y, x| {
        let base = if y < stride {
            y
        } else {
            ((y % stride) + 1) * stride
        };
        base + (x * (stride + 1))
    };
    result += fun_name(pattern, &map, index_fn, range, range_fn);

    // NORTH-WEST
    let range = 0..=diag;
    let range_fn = |y| (0..(stride - (y % stride) - (y / stride))).rev();
    let index_fn = |y, x| {
        let base = if y < stride {
            y
        } else {
            ((y % stride) + 1) * stride
        };
        base + (x * (stride + 1))
    };
    result += fun_name(pattern, &map, index_fn, range, range_fn);

    eprint!("{result}")
}

fn fun_name<I: Iterator<Item = usize>, J: Iterator<Item = usize>>(
    pattern: &'static str,
    map: &[char],
    index_fn: impl Fn(usize, usize) -> usize,
    range: J,
    range_fn: impl Fn(usize) -> I,
) -> usize {
    let mut result = 0;
    let mut looking_for = pattern.chars();
    let pattern_start = looking_for.next().unwrap();
    let mut reading = false;

    //print!("{DIM}");
    for x in range {
        //print!("{DIM}");
        let result_old = result;
        for y in range_fn(x) {
            let index = index_fn(x, y);
            let cursor = map[index];
            if reading {
                match looking_for.next() {
                    Some(letter) => {
                        if letter == cursor {
                            //print!("{RESET}{cursor}");
                            continue;
                        }
                    }
                    None => result += 1,
                };
                looking_for = pattern.chars();
                _ = looking_for.next();
            }
            let reading_old = reading;
            reading = cursor == pattern_start;

            //print!("{}{cursor}", if reading { RESET } else { DIM });
        }
        if looking_for.next().is_none() {
            result += 1;
        }
        let added = result - result_old;
        //print!("{RESET}");
        if added > 0 {
            //println!("  <-- {added}");
        } else {
            //println!();
        }
        reading = false;
        looking_for = pattern.chars();
        _ = looking_for.next();
    }
    //println!();
    result
}

fn part2() {
    let stride = INPUT.find('\n').unwrap();
    let map: Vec<_> = INPUT.lines().flat_map(|x| x.chars()).collect();
    let mut result = 0;

    //  NOTE(matyas): We'll be looking for the "A"s and testing if there's an x around them.
    // That would be impossible for all "A"s on the edges so we don't need to look at those
    for y in 1..stride - 1 {
        'row: for x in 1..stride - 1 {
            if map[y * stride + x] == 'A' {
                for x_offset in [-1, 1] {
                    for y_offset in [-1, 1] {
                        let new_y = (y as isize + y_offset) as usize;
                        let new_x = (x as isize + x_offset) as usize;
                        if map[new_y * stride + new_x] == 'M' {
                            // Found an M on one of the corners
                            let new_y = (y as isize - y_offset) as usize;
                            let new_x = (x as isize - x_offset) as usize;
                            if map[new_y * stride + new_x] == 'S' {
                                // Found an S opposite the M
                                let new_y = (y as isize - y_offset) as usize;
                                let new_x = (x as isize + x_offset) as usize;
                                if map[new_y * stride + new_x] == 'M' {
                                    // Found a second M horizontally from the first
                                    let new_y = (y as isize + y_offset) as usize;
                                    let new_x = (x as isize - x_offset) as usize;
                                    if map[new_y * stride + new_x] == 'S' {
                                        // Bingo
                                        result += 1;
                                        continue 'row;
                                    }
                                } else {
                                    let new_y = (y as isize + y_offset) as usize;
                                    let new_x = (x as isize - x_offset) as usize;
                                    if map[new_y * stride + new_x] == 'M' {
                                        let new_y = (y as isize - y_offset) as usize;
                                        let new_x = (x as isize + x_offset) as usize;
                                        if map[new_y * stride + new_x] == 'S' {
                                            // Bingo
                                            result += 1;
                                            continue 'row;
                                        }

                                        // Found a second M vertically from the first
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{result}");
}
