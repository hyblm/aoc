fn main() {
    let input = include_str!("input.txt");
    let predictions_sum: isize = input.lines().map(predict_previous_value).sum();

    println!("{predictions_sum}");
}

fn predict_previous_value(line: &str) -> isize {
    let mut starts = Vec::new();
    let mut sign = 1;
    let mut append = |x: isize| {
        starts.push(sign * x);
        sign *= -1
    };

    let values = line.split_whitespace();
    let mut values: Vec<isize> = values
        .into_iter()
        .map(|x| x.parse().expect("integer"))
        .collect();

    append(*values.first().expect("not empty"));
    eprintln!("{values:?}");

    let mut steps = Vec::with_capacity(values.len() - 1);
    for pair in values.windows(2) {
        steps.push(pair[1] - pair[0]);
    }
    append(*steps.first().expect("not empty"));
    eprintln!("{steps:?}");

    while !steps.iter().all(|&x| x == 0) {
        values.clear();
        values.extend_from_slice(&steps);
        steps.clear();
        for pair in values.windows(2) {
            steps.push(pair[1] - pair[0]);
        }
        append(*steps.first().expect("not empty"));
        eprintln!("{steps:?}");
    }

    let result = starts.iter().sum();
    eprintln!("starts: {starts:?} = {result}");
    result
}

fn predict_next_value(line: &str) -> isize {
    let mut ends = Vec::new();

    let values = line.split_whitespace();
    let mut values: Vec<isize> = values
        .into_iter()
        .map(|x| x.parse().expect("integer"))
        .collect();
    ends.push(*values.last().expect("not empty"));

    let mut steps = Vec::with_capacity(values.len() - 1);
    for pair in values.windows(2) {
        steps.push(pair[1] - pair[0]);
    }
    ends.push(*steps.last().expect("not empty"));

    while !steps.iter().all(|&x| x == 0) {
        values.clear();
        values.extend_from_slice(&steps);
        steps.clear();
        for pair in values.windows(2) {
            steps.push(pair[1] - pair[0]);
        }
        ends.push(*steps.last().expect("not empty"));
    }

    ends.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_next() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let predictions_sum: isize = input.lines().map(predict_next_value).sum();
        assert_eq!(predictions_sum, 114);
    }

    #[test]
    fn example_previous() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let predictions_sum: isize = input.lines().map(predict_previous_value).sum();
        assert_eq!(predictions_sum, 2);
    }
}
