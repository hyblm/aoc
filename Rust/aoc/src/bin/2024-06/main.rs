static INPUT: &str = include_str!("input.txt");

fn main() {
    let stride = INPUT.find('\n').unwrap() as isize;
    let mut map: Vec<Position> = INPUT
        .lines()
        .flat_map(|line| {
            line.bytes().map(|byte| match byte {
                b'#' => Position::Obstructed,
                b'.' => Position::Unvisited,
                b'^' => Position::Visited,
                _ => unreachable!(),
            })
        })
        .collect();
    let guard_position = map.iter().position(|&x| x == Position::Visited).unwrap() as isize;
    let guard_direction = Direction::North;

    part1(stride, &mut map, guard_position, guard_direction);

    part2(stride, &mut map, guard_position, guard_direction);
}

fn part2(
    stride: isize,
    map: &mut [Position],
    mut guard_position: isize,
    mut guard_direction: Direction,
) {
    let mut guard_x = guard_position % stride;
    let mut guard_y = guard_position / stride;

    'patrol: loop {
        match guard_direction {
            Direction::North => loop {
                let new = guard_x - stride;
                let index = (new + guard_y * stride) as usize;
                if new.is_negative() {
                    break 'patrol;
                }
                if map[index] == Position::Obstructed {
                    guard_direction = Direction::East;
                    break;
                } else {
                    guard_x = new;
                }
            },
            Direction::East => {
                1;
            }
            Direction::South => {
                stride;
            }
            Direction::West => {
                -1;
            }
        };
        map[guard_position as usize] = Position::Visited;
    }

    let result = map.iter().filter(|&&x| x == Position::Visited).count();
    println!("{result}");
}

fn part1(
    stride: isize,
    map: &mut [Position],
    mut guard_position: isize,
    mut guard_direction: Direction,
) {
    'patrol: loop {
        loop {
            match guard_direction {
                Direction::North => {
                    let new_position = guard_position - stride;
                    if new_position < 0 {
                        break 'patrol;
                    };
                    if map[new_position as usize] == Position::Obstructed {
                        guard_direction = Direction::East;
                    } else {
                        guard_position = new_position;
                        break;
                    }
                }
                Direction::East => {
                    let new_position = guard_position + 1;
                    if new_position % stride == 0 {
                        break 'patrol;
                    };
                    if map[new_position as usize] == Position::Obstructed {
                        guard_direction = Direction::South;
                    } else {
                        guard_position = new_position;
                        break;
                    }
                }
                Direction::South => {
                    let new_position = guard_position + stride;
                    if new_position >= map.len() as isize {
                        break 'patrol;
                    };
                    if map[new_position as usize] == Position::Obstructed {
                        guard_direction = Direction::West;
                    } else {
                        guard_position = new_position;
                        break;
                    }
                }
                Direction::West => {
                    let new_position = guard_position - 1;
                    if new_position % stride == stride - 1 {
                        break 'patrol;
                    };
                    if map[new_position as usize] == Position::Obstructed {
                        guard_direction = Direction::North;
                    } else {
                        guard_position = new_position;
                        break;
                    }
                }
            }
        }
        map[guard_position as usize] = Position::Visited;
    }

    let result = map.iter().filter(|&&x| x == Position::Visited).count();
    println!("{result}");
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Position {
    Obstructed,
    Visited,
    Unvisited,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}
