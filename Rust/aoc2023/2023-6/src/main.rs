fn main() {
    let (times, distances) = include_str!("input.txt").split_once("\n").unwrap();
    let times = times.split_at(11).1;
    let distances = distances.split_at(11).1;
    let time: String = times.chars().filter(|&x| x.is_digit(10)).collect();
    let time = time.parse::<usize>().unwrap();
    let record: String = (distances).chars().filter(|&x| x.is_digit(10)).collect();
    let record = record.parse::<usize>().unwrap();
    let times = str_to_vec(times);
    let distances = str_to_vec(distances);

    println!("part one: {}", part_one(&times, &distances));

    let hold_time = find_bounds(time, record);
    let ways_to_win = time - 2 * hold_time + 1;
    println!("part two: {ways_to_win}");
}

fn part_one(times: &[usize], distances: &[usize]) -> usize {
    times
        .iter()
        .zip(distances)
        .map(|(&time, &record)| {
            let hold_time = find_bounds(time, record);
            let ways_to_win = time - 2 * hold_time + 1;
            ways_to_win
        })
        .product()
}

fn str_to_vec(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

fn find_bounds(time: usize, record: usize) -> usize {
    let (mut left, mut right) = (0usize, time);

    right = loop {
        let hold_time = (left + right) / 2;
        let distance = distance(time, hold_time);
        if distance > record {
            break hold_time;
        }
    };

    while left + 1 != right {
        let hold_time = (left + right) / 2;
        let distance = distance(time, hold_time);
        if distance > record {
            right = hold_time;
        } else {
            left = hold_time;
        }
    }

    right
}

fn distance(time: usize, hold_time: usize) -> usize {
    let time_left = time - hold_time;
    time_left * hold_time
}
