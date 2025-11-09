use nom::{branch::alt, bytes::complete::tag_no_case, IResult};
fn main() {
    let part1: u32 = include_str!("input.txt")
        .lines()
        .map(|line| {
            let nums: Vec<_> = line.chars().filter_map(|x| x.to_digit(10)).collect();
            nums.first().unwrap() * 10 + nums.last().unwrap()
        })
        .sum();
    dbg!(&part1);

    let part2: u32 = include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut first = 0;
            let mut last = 0;

            let mut i = line;
            while !i.is_empty() {
                let parsed = match parse_digit(i) {
                    Ok((rest, num)) => Some((rest, num)),
                    Err(_) => {
                        let (num, rest) = i.split_at(1);
                        num.parse::<u32>().ok().map(|x| (rest, x))
                    }
                };
                match parsed {
                    Some((rest, num)) => {
                        i = rest;
                        if first == 0 {
                            first = num;
                        }
                        last = num;
                    }
                    None => {
                        let (_, rest) = i.split_at(1);
                        i = rest;
                    }
                }
            }
            first * 10 + last
        })
        .sum();
    dbg!(&part2);
}

fn parse_digit(i: &str) -> IResult<&str, u32> {
    let (_, x) = alt((
        tag_no_case("one"),
        tag_no_case("two"),
        tag_no_case("three"),
        tag_no_case("four"),
        tag_no_case("five"),
        tag_no_case("six"),
        tag_no_case("seven"),
        tag_no_case("eight"),
        tag_no_case("nine"),
    ))(i)?;

    let num: u32 = match x {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => unreachable!(),
    };
    Ok((&i[1..], num))
}
