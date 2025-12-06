#![allow(unused)]

static INPUT: &str = include_str!("input.txt");
fn main() {
    let input = parse_input(INPUT);
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

static DIAL_MAX: usize = 100;
static DIAL_INITIAL: usize = 50;

#[derive(Debug, Clone, Copy)]
enum Rotation {
    Left { distance: usize },  // current position decreases
    Right { distance: usize }, // current position encreases
}

fn parse_input(input: &str) -> Vec<Rotation> {
    input
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
                Rotation::Left { distance } => DIAL_MAX - (distance % DIAL_MAX),
                Rotation::Right { distance } => *distance,
            };
            *dial %= DIAL_MAX;
            Some(*dial)
        })
        .filter(|dial| *dial == 0)
        .count()
}

fn part2(rotations: &[Rotation]) -> usize {
    rotations
        .iter()
        .scan(DIAL_INITIAL, |dial, rotation| {
            Some(apply_rotation_and_count_zeroes(dial, *rotation))
        })
        .sum()
}

fn apply_rotation_and_count_zeroes(dial: &mut usize, rotation: Rotation) -> usize {
    let (distance_from_zero, apply_distance) = match rotation {
        Rotation::Left { distance } => (
            (DIAL_MAX - *dial) % DIAL_MAX + distance,
            DIAL_MAX - (distance % DIAL_MAX),
        ),
        Rotation::Right { distance } => (*dial + distance, distance),
    };
    *dial += apply_distance;
    *dial %= DIAL_MAX;

    distance_from_zero / DIAL_MAX
}

fn brute_force(dial: &mut usize, rotation: Rotation) -> usize {
    let mut zeros_seen = 0;
    match rotation {
        Rotation::Left { distance } => {
            let step = |x| if x == 0 { DIAL_MAX - 1 } else { x - 1 };
            for _ in 0..distance {
                *dial = step(*dial);
                if *dial == 0 {
                    zeros_seen += 1;
                }
            }
        }
        Rotation::Right { distance } => {
            let step = |x| if x == (DIAL_MAX - 1) { 0 } else { x + 1 };
            for _ in 0..distance {
                *dial = step(*dial);
                if *dial == 0 {
                    zeros_seen += 1;
                }
            }
        }
    };
    zeros_seen
}

#[cfg(test)]
mod test {
    mod left {
        use crate::{DIAL_INITIAL, DIAL_MAX, Rotation, apply_rotation_and_count_zeroes};
        #[test]
        fn starting_at_zero_zero() {
            let mut dial = 0;
            let seen_zero =
                apply_rotation_and_count_zeroes(&mut dial, Rotation::Left { distance: 0 });
            assert_eq!(seen_zero, 0);
            assert_eq!(dial, 0);
        }

        #[test]
        fn starting_at_zero_less_than_full_rotation() {
            let mut dial = 0;
            let seen_zero = apply_rotation_and_count_zeroes(
                &mut dial,
                Rotation::Left {
                    distance: DIAL_MAX / 2,
                },
            );
            assert_eq!(seen_zero, 0);
            assert_eq!(dial, DIAL_MAX / 2);
        }

        #[test]
        fn starting_at_zero_multiple_turns_lands_on_zero() {
            let mut dial = 0;
            let seen_zero = apply_rotation_and_count_zeroes(
                &mut dial,
                Rotation::Left {
                    distance: 5 * DIAL_MAX,
                },
            );
            assert_eq!(seen_zero, 5);
            assert_eq!(dial, 0);
        }

        #[test]
        fn starting_at_zero_multiple_turns_overshoots() {
            let mut dial = 0;
            let seen_zero =
                apply_rotation_and_count_zeroes(&mut dial, Rotation::Left { distance: 560 });
            assert_eq!(seen_zero, 5);
            assert_eq!(dial, DIAL_MAX - (560 % DIAL_MAX));
        }

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
