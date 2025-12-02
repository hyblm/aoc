#![allow(unused)]

static INPUT: &str = include_str!("input.txt");

static DIAL_MIN: usize = 0;
static DIAL_MAX: usize = 100;
static DIAL_INITIAL: usize = 50;

#[derive(Debug, Clone, Copy)]
enum Rotation {
    Left { distance: usize },  // current position decreases
    Right { distance: usize }, // current position encreases
}

fn main() {
    let input = parse_input();
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn parse_input() -> Vec<Rotation> {
    INPUT
        .lines()
        .map(|line| {
            let (direction, distance) = line.split_at(1);
            let distance = distance.parse().unwrap();
            match direction {
                "L" => Rotation::Left { distance },
                "R" => Rotation::Right { distance },
                _ => unreachable!(),
            }
        })
        .collect()
}

fn part1(rotations: &[Rotation]) -> usize {
    rotations
        .iter()
        .scan(DIAL_INITIAL, |dial, rotation| {
            *dial += match rotation {
                Rotation::Left { distance } => {
                    (DIAL_MAX * (((distance / DIAL_MAX) * 2) + 1)) - distance
                }
                Rotation::Right { distance } => *distance,
            };
            *dial %= DIAL_MAX;
            Some(*dial)
        })
        .filter(|dial| *dial == 0)
        .count()
}

fn part2(rotations: &[Rotation]) -> usize {
    let mut zero_count = 0;
    let mut dial = DIAL_INITIAL;
    for rotation in rotations {
        zero_count += apply_rotation_and_count_zeroes(&mut dial, *rotation);
    }
    zero_count
}

fn brute_force(dial: &mut usize, rotation: Rotation) -> usize {
    let mut zeros_seen = 0;
    match rotation {
        Rotation::Left { distance } => {
            let step = |x| if x == 0 { DIAL_MAX - 1 } else { x - 1 };
            for i in 0..distance {
                *dial = step(*dial);
                if *dial == 0 {
                    zeros_seen += 1;
                }
            }
        }
        Rotation::Right { distance } => {
            let step = |x| if x == (DIAL_MAX - 1) { 0 } else { x + 1 };
            for i in 0..distance {
                *dial = step(*dial);
                if *dial == 0 {
                    zeros_seen += 1;
                }
            }
        }
    };
    zeros_seen
}
fn apply_rotation_and_count_zeroes(dial: &mut usize, rotation: Rotation) -> usize {
    // let seen_zero = match rotation {
    //     Rotation::Left { distance } => {
    //         let mut seen_zero = distance / DIAL_MAX;
    //         let left_to_rotate = distance % DIAL_MAX;
    //         match left_to_rotate.cmp(dial) {
    //             std::cmp::Ordering::Less => *dial -= left_to_rotate,
    //             std::cmp::Ordering::Equal => {
    //                 seen_zero += 1;
    //                 *dial = 0;
    //             }
    //             std::cmp::Ordering::Greater => {
    //                 seen_zero += 1;
    //                 *dial = DIAL_MAX - (left_to_rotate - *dial);
    //             }
    //         }
    //         seen_zero
    //
    //         // if distance == 0 {
    //         //     return 0;
    //         // };
    //         // let overturns = distance / DIAL_MAX;
    //         // *dial += DIAL_MAX - (distance % DIAL_MAX);
    //         // if *dial <= DIAL_MAX {
    //         //     overturns + 1
    //         // } else {
    //         //     overturns
    //         // }
    //     }
    //     Rotation::Right { distance } => {
    //         *dial += distance;
    //         *dial / DIAL_MAX
    //     }
    // };
    // *dial %= DIAL_MAX;
    zeros_seen
}

#[cfg(test)]
mod test {
    use crate::{DIAL_INITIAL, Rotation, apply_rotation_and_count_zeroes};

    mod left {
        use crate::{DIAL_INITIAL, Rotation, apply_rotation_and_count_zeroes};

        #[test]
        fn zero() {
            let mut dial = DIAL_INITIAL;
            let seen_zero =
                apply_rotation_and_count_zeroes(&mut dial, Rotation::Left { distance: 0 });
            assert_eq!(seen_zero, 0);
            assert_eq!(dial, DIAL_INITIAL);
        }

        #[test]
        fn less_than_dial() {
            let mut dial = DIAL_INITIAL;
            let seen_zero =
                apply_rotation_and_count_zeroes(&mut dial, Rotation::Left { distance: 10 });
            assert_eq!(seen_zero, 0);
            assert_eq!(dial, DIAL_INITIAL - 10);
        }

        #[test]
        fn equal_to_dial() {
            let mut dial = DIAL_INITIAL;
            let seen_zero = apply_rotation_and_count_zeroes(
                &mut dial,
                Rotation::Left {
                    distance: DIAL_INITIAL,
                },
            );
            assert_eq!(seen_zero, 1);
            assert_eq!(dial, 0);
        }

        #[test]
        fn greater_than_dial() {
            let mut dial = DIAL_INITIAL;
            let seen_zero =
                apply_rotation_and_count_zeroes(&mut dial, Rotation::Left { distance: 60 });
            assert_eq!(seen_zero, 1);
            assert_eq!(dial, 90);
        }

        #[test]
        fn zero_with_overturn() {
            let mut dial = DIAL_INITIAL;
            let seen_zero =
                apply_rotation_and_count_zeroes(&mut dial, Rotation::Left { distance: 500 });
            assert_eq!(seen_zero, 5);
            assert_eq!(dial, DIAL_INITIAL);
        }

        #[test]
        fn less_than_dial_with_overturn() {
            let mut dial = DIAL_INITIAL;
            let seen_zero =
                apply_rotation_and_count_zeroes(&mut dial, Rotation::Left { distance: 510 });
            assert_eq!(seen_zero, 5);
            assert_eq!(dial, DIAL_INITIAL - 10);
        }

        #[test]
        fn equal_to_dial_with_overturn() {
            let mut dial = DIAL_INITIAL;
            let seen_zero = apply_rotation_and_count_zeroes(
                &mut dial,
                Rotation::Left {
                    distance: DIAL_INITIAL + 500,
                },
            );
            assert_eq!(seen_zero, 6);
            assert_eq!(dial, 0);
        }

        #[test]
        fn greater_than_dial_with_overturn() {
            let mut dial = DIAL_INITIAL;
            let seen_zero =
                apply_rotation_and_count_zeroes(&mut dial, Rotation::Left { distance: 560 });
            assert_eq!(seen_zero, 6);
            assert_eq!(dial, 90);
        }
    }
    mod right {
        use crate::{DIAL_INITIAL, Rotation, apply_rotation_and_count_zeroes};

        #[test]
        fn zero() {
            let mut dial = DIAL_INITIAL;
            let seen_zero =
                apply_rotation_and_count_zeroes(&mut dial, Rotation::Right { distance: 0 });
            assert_eq!(seen_zero, 0);
            assert_eq!(dial, DIAL_INITIAL);
        }

        #[test]
        fn less_than_dial() {
            let mut dial = DIAL_INITIAL;
            let seen_zero =
                apply_rotation_and_count_zeroes(&mut dial, Rotation::Right { distance: 10 });
            assert_eq!(seen_zero, 0);
            assert_eq!(dial, DIAL_INITIAL + 10);
        }

        #[test]
        fn equal_to_dial() {
            let mut dial = DIAL_INITIAL;
            let seen_zero = apply_rotation_and_count_zeroes(
                &mut dial,
                Rotation::Right {
                    distance: DIAL_INITIAL,
                },
            );
            assert_eq!(seen_zero, 1);
            assert_eq!(dial, 0);
        }

        #[test]
        fn greater_than_dial() {
            let mut dial = DIAL_INITIAL;
            let seen_zero =
                apply_rotation_and_count_zeroes(&mut dial, Rotation::Right { distance: 60 });
            assert_eq!(seen_zero, 1);
            assert_eq!(dial, 10);
        }

        #[test]
        fn zero_with_overturn() {
            let mut dial = DIAL_INITIAL;
            let seen_zero =
                apply_rotation_and_count_zeroes(&mut dial, Rotation::Right { distance: 500 });
            assert_eq!(seen_zero, 5);
            assert_eq!(dial, DIAL_INITIAL);
        }

        #[test]
        fn less_than_dial_with_overturn() {
            let mut dial = DIAL_INITIAL;
            let seen_zero =
                apply_rotation_and_count_zeroes(&mut dial, Rotation::Right { distance: 510 });
            assert_eq!(seen_zero, 5);
            assert_eq!(dial, DIAL_INITIAL + 10);
        }

        #[test]
        fn equal_to_dial_with_overturn() {
            let mut dial = DIAL_INITIAL;
            let seen_zero = apply_rotation_and_count_zeroes(
                &mut dial,
                Rotation::Right {
                    distance: DIAL_INITIAL + 500,
                },
            );
            assert_eq!(seen_zero, 6);
            assert_eq!(dial, 0);
        }

        #[test]
        fn greater_than_dial_with_overturn() {
            let mut dial = DIAL_INITIAL;
            let seen_zero =
                apply_rotation_and_count_zeroes(&mut dial, Rotation::Right { distance: 560 });
            assert_eq!(seen_zero, 6);
            assert_eq!(dial, 10);
        }
    }
}
