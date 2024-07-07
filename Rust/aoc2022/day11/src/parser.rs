use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{one_of, space1},
    combinator::{map, value},
    sequence::{preceded, tuple},
    IResult,
};

use crate::{Monkey, Operation, Term};

pub fn parse_term(i: &str) -> IResult<&str, Term> {
    alt((
        value(Term::Old, tag("old")),
        map(nom::character::complete::u64, Term::Constant),
    ))(i)
}

pub fn parse_operation(i: &str) -> IResult<&str, Operation> {
    let (i, (l, op, r)) =
        preceded(tag("new = "), tuple((parse_term, one_of("*+"), parse_term)))(i)?;

    let op = match op {
        '*' => Operation::Add(l, r),
        '+' => Operation::Mul(l, r),
        _ => unreachable!(),
    };

    Ok((i, op))
}

pub fn parse_monkey(i: &str) -> IResult<&str, Monkey> {
    let (i, _) = tuple((
        space1,
        tag("Monkey "),
        nom::character::complete::u64,
        tag(":\n"),
    ))(i)?;

    todo!()
}
