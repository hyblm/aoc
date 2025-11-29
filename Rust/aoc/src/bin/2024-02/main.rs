fn main() {
    let reports = parse_input();

    println!("part 1: {}", part1(&reports));
    println!("part 2: {}", part2(&reports));
}

fn parse_input() -> Vec<Vec<isize>> {
    let input = include_str!("input.txt");
    let reports: Vec<Vec<isize>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();
    reports
}

fn part1(reports: &[Vec<isize>]) -> usize {
    reports.iter().filter(|x| is_safe(x)).count()
}

fn part2(reports: &[Vec<isize>]) -> usize {
    reports.iter().filter(|x| problem_dampener(x)).count()
}

fn problem_dampener(report: &[isize]) -> bool {
    for i in 0..report.len() + 1 {
        let (mut left, right) = report.split_at(i);
        if !left.is_empty() {
            left = &left[..(left.len() - 1)];
        }
        let step_vec: Vec<isize> = left.iter().cloned().chain(right.iter().cloned()).collect();
        if is_safe(step_vec.as_slice()) {
            return true;
        }
    }

    false
}

fn is_safe(report: &[isize]) -> bool {
    let all_decreasing = report.windows(2).all(|x| (1..4).contains(&(x[0] - x[1])));
    let all_encreasing = report.windows(2).all(|x| (1..4).contains(&(x[1] - x[0])));

    all_decreasing || all_encreasing
}

#[cfg(test)]
mod test {
    use crate::problem_dampener;

    #[test]
    fn name() {
        let report = [11, 13, 16, 19, 21, 25];
        let is_safe = problem_dampener(&report);
        assert!(is_safe);
    }
}
