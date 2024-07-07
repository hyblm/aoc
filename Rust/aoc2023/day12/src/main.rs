use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete as cc;
use nom::character::complete::char;
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::sequence::separated_pair;
use nom::IResult;

fn main() {
    let (_, records) = parse_input(include_str!("test.txt")).unwrap();
    let num_arrangements: usize = records.iter().map(Record::arrangements).sum();

    println!("part one: {num_arrangements}");
}

#[derive(Debug)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl Condition {
    pub fn new(c: char) -> Self {
        match c {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => unreachable!(),
        }
    }
}

fn condition(i: &str) -> IResult<&str, Condition> {
    map(alt((char('.'), char('#'), char('?'))), Condition::new)(i)
}

#[derive(Debug)]
struct Record {
    springs: Vec<Condition>,
    groups: Vec<u32>,
}

impl Record {
    pub fn new(springs: Vec<Condition>, groups: Vec<u32>) -> Self {
        Self { springs, groups }
    }

    // pub fn matches(springs: &[Condition], groups: &[u32]) -> bool {
    //     let mut group
    //     for spring in springs {
    //         if let Condition::Damaged = spring {

    //         }
    //     }
    // }

    pub fn arrangements(self: &Self) -> usize {
        if let Some (sub_index )= self
            .springs
            .iter()
            .position(|x| matches!(x, Condition::Unknown));
        // Substitute unknown in springs
        // test if matches groups
        // Record::matches(springs, groups);
        todo!()
    }
}

fn record(i: &str) -> IResult<&str, Record> {
    map(
        separated_pair(
            many1(condition),
            tag(" "),
            separated_list1(tag(","), cc::u32),
        ),
        |(springs, groups)| Record::new(springs, groups),
    )(i)
}

fn parse_input(i: &str) -> IResult<&str, Vec<Record>> {
    separated_list1(tag("\n"), record)(i)
}
