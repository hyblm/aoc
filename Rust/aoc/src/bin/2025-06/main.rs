static INPUT: &str = include_str!("input.txt");
fn main() {
    let input = parse_input(INPUT);
    println!("part 1: {}", compute(&input));
    let input = parse_input2(INPUT);
    println!("part 2: {}", compute(&input));
}

#[derive(Debug)]
enum Op {
    Add,
    Mul,
}
#[derive(Debug)]
struct Problem {
    op: Op,
    numbers: Vec<usize>,
}

fn parse_input(input: &str) -> Vec<Problem> {
    let mut iter = input.lines().rev();

    let first_line = iter.next().unwrap();
    let mut problems: Vec<Problem> = first_line
        .split_whitespace()
        .map(|c| Problem {
            op: match c {
                "*" => Op::Mul,
                "+" => Op::Add,
                _ => unreachable!(),
            },
            numbers: Vec::new(),
        })
        .collect();

    for line in iter {
        for (num_str, problem) in line.split_whitespace().zip(&mut problems) {
            let num: usize = num_str.parse().unwrap();
            problem.numbers.push(num);
        }
    }

    problems
}

fn parse_input2(input: &str) -> Vec<Problem> {
    let mut iter = input.lines().rev();

    let first_line = iter.next().unwrap();
    let mut start_positions = Vec::new();
    let mut problems: Vec<Problem> = first_line
        .split_whitespace()
        .map(|c| {
            start_positions.push(c.as_ptr().addr() - first_line.as_ptr().addr());

            Problem {
                op: match c {
                    "*" => Op::Mul,
                    "+" => Op::Add,
                    _ => unreachable!(),
                },
                numbers: Vec::new(),
            }
        })
        .collect();

    for line in iter.rev() {
        for ((num_str, problem), p) in line
            .split_whitespace()
            .zip(&mut problems)
            .zip(&start_positions)
        {
            let offset_in_line = num_str.as_ptr().addr() - line.as_ptr().addr();
            let index_in_problem = offset_in_line - p;

            while index_in_problem >= problem.numbers.len() {
                problem.numbers.push(0);
            }

            for (i, num) in num_str.bytes().map(|x| x - b'0').enumerate() {
                let index = i + index_in_problem;
                if index < problem.numbers.len() {
                    problem.numbers[index] *= 10;
                    problem.numbers[index] += usize::from(num);
                } else {
                    problem.numbers.push(usize::from(num));
                }
            }
        }
    }

    problems
}

fn compute(problems: &[Problem]) -> usize {
    problems
        .iter()
        .map(|p| match p.op {
            Op::Add => p.numbers.iter().sum::<usize>(),
            Op::Mul => p.numbers.iter().product(),
        })
        .sum()
}
