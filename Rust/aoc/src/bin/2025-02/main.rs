#![allow(unused)]

use std::ops::{ControlFlow, RangeInclusive};

static INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input();
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn parse_input() -> Vec<(usize, usize)> {
    INPUT
        .trim()
        .split(',')
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            let start: usize = start.parse().unwrap();
            let end: usize = end.parse().unwrap();
            (start, end)
        })
        .collect()
}

fn part1(ranges: &[(usize, usize)]) -> usize {
    let mut invalid_id_sum = 0;
    for range in ranges {
        for id in range.0..=range.1 {
            let digit_count = count_digits(id);
            if !digit_count.is_multiple_of(2) {
                continue;
            }
            let sentinel: usize = 10usize.pow(digit_count / 2) + 1;
            if id.is_multiple_of(sentinel) {
                invalid_id_sum += id;
            }
        }
    }
    invalid_id_sum
}

fn count_digits(start: usize) -> u32 {
    let mut exponent = 0;
    let mut radix = 10;
    let mut divisor = 1;
    while (start / divisor) > 0 {
        exponent += 1;
        divisor *= radix;
    }
    exponent
}

fn part2(ranges: &[(usize, usize)]) -> usize {
    let mut invalid_id_sum = 0;
    for range in ranges {
        'id_check: for id in range.0..=range.1 {
            let digit_count = count_digits(id);
            for repetition in 2..=digit_count {
                if digit_count.is_multiple_of(repetition)
                    && let Some(invalid_id) = fun_name(id, digit_count, repetition)
                {
                    invalid_id_sum += invalid_id;
                    continue 'id_check;
                };
            }
        }
    }
    invalid_id_sum
}

fn fun_name(id: usize, digit_count: u32, repetition_count: u32) -> Option<usize> {
    let exp = digit_count / repetition_count;
    let sentinel = {
        let mut sentinel = 1;
        for i in 1..repetition_count {
            sentinel += 10usize.pow(i * exp)
        }
        sentinel
    };
    if id.is_multiple_of(sentinel) {
        return Some(id);
    }
    None
}
