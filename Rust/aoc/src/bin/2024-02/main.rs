fn main() {
    part1();
    part2();
}

fn part1() {
    let input = include_str!("input.txt");
    let mut safe_count = 0;
    let mut report = Vec::new();

    for line in input.lines() {
        let levels = line.split_whitespace().map(|x| x.parse::<isize>().unwrap());
        report.extend(levels);

        if is_safe(&report) {
            safe_count += 1;
        }
        report.clear();
    }

    println!("{safe_count}");
}

fn part2() {
    let input = include_str!("input.txt");
    let mut safe_count = 0;
    let mut report = Vec::new();

    for line in input.lines() {
        let levels = line.split_whitespace().map(|x| x.parse::<isize>().unwrap());
        report.extend(levels);

        let report_is_safe = problem_dampener(&report);
        if report_is_safe {
            safe_count += 1;
        } else {
            eprintln!("{report:?}");
        }

        report.clear();
    }

    println!("{safe_count}");
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
    let steps = report.windows(2).map(|x| x[0] - x[1]);
    let is_gradual = steps.clone().all(|x| x.abs() < 4);
    let transition_count = report.len().saturating_sub(1);
    let decreasing_count = steps.clone().filter(|x| x.is_positive()).count();
    let equal_count = steps.clone().filter(|x| *x == 0).count();
    let is_monotonic = decreasing_count == 0 || decreasing_count == transition_count;
    let is_monotonic = is_monotonic && equal_count == 0;

    is_gradual && is_monotonic
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
