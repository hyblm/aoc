fn main() {
    let input = include_str!("input.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) -> u32 {
    let max_set = CubesSet {
        red: 12,
        green: 13,
        blue: 14,
    };
    let part1: u32 = input
        .lines()
        .filter_map(|l| -> Option<u32> {
            let (_, (id, game)) = parser::game(l).unwrap();
            if game.iter().all(|set| set.matches(max_set)) {
                Some(id)
            } else {
                None
            }
        })
        .sum();
    dbg!(part1)
}
fn part2(input: &str) -> u32 {
    let part1: u32 = input
        .lines()
        .map(|l| {
            let (_, (_, game)) = parser::game(l).unwrap();
            {
                game.into_iter()
                    .reduce(|max, cube| CubesSet {
                        red: max.red.max(cube.red),
                        green: max.green.max(cube.green),
                        blue: max.blue.max(cube.blue),
                    })
                    .expect("non empty")
            }
            .product()
        })
        .sum();
    dbg!(part1)
}

#[derive(Debug, Copy, Clone)]
struct CubesSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubesSet {
    fn matches(self, other: CubesSet) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }
    fn product(self) -> u32 {
        self.red * self.green * self.blue
    }
}

mod parser {
    use nom::branch::alt;
    use nom::character::complete as cc;
    use nom::multi::separated_list1;
    use nom::sequence::{preceded, separated_pair, tuple};
    use nom::{bytes::complete::tag, sequence::delimited};

    use super::CubesSet;

    use nom::IResult;

    pub(crate) fn game(input: &str) -> IResult<&str, (u32, Vec<CubesSet>)> {
        tuple((game_id, rounds))(input)
    }

    pub(crate) fn rounds(input: &str) -> IResult<&str, Vec<CubesSet>> {
        separated_list1(tag(";"), round)(input)
    }

    fn round(input: &str) -> IResult<&str, CubesSet> {
        let (rest, round) = separated_list1(tag(","), cube)(input)?;
        debug_assert!(round.len() <= 3);

        let (mut red, mut green, mut blue) = (0, 0, 0);
        for (count, color) in round {
            match color {
                "red" => red = count,
                "green" => green = count,
                "blue" => blue = count,
                _ => unreachable!(),
            }
        }

        let cubes = CubesSet { red, green, blue };
        Ok((rest, cubes))
    }

    fn cube(input: &str) -> IResult<&str, (u32, &str)> {
        preceded(
            tag(" "),
            separated_pair(
                cc::u32,
                tag(" "),
                alt((tag("blue"), tag("green"), tag("red"))),
            ),
        )(input)
    }

    pub(crate) fn game_id(input: &str) -> IResult<&str, u32> {
        delimited(tag("Game "), cc::u32, tag(":"))(input)
    }
}
