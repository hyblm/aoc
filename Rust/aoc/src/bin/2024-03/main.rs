static INPUT: &str = include_str!("input.txt");

fn main() {
    part1();
    part2();
}

fn part1() {
    let instructions = INPUT;
    let result = process_muls(instructions);
    println!("{result}");
}

fn part2() {
    let mut result = 0;

    let mut split = INPUT.split("don't()");
    let instructions = split.next().unwrap();
    result += process_muls(instructions);

    for disabled in split {
        let Some((_, enabled)) = disabled.split_once("do()") else {
            continue;
        };
        result += process_muls(enabled);
    }
    println!("{result}");
}

fn process_muls(instructions: &str) -> isize {
    let mut result = 0;
    for line in instructions.split("mul(").skip(1) {
        let Some((lhs, rhs)) = line.split_once(',') else {
            continue;
        };
        let Some((rhs, _)) = rhs.split_once(')') else {
            continue;
        };
        let Ok(rhs) = rhs.parse::<isize>() else {
            continue;
        };
        let Ok(lhs) = lhs.parse::<isize>() else {
            continue;
        };

        result += lhs * rhs;
    }
    result
}
