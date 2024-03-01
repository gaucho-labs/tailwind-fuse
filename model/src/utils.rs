use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map_res, opt, recognize},
    sequence::tuple,
    IResult,
};
use std::str::FromStr;

/// `\d+\.\d+`
pub fn parse_f32(input: &str) -> IResult<&str, f32> {
    let float1 = tuple((digit1, opt(tuple((tag("."), digit1)))));
    map_res(recognize(float1), str::parse)(input)
}

pub fn parse_fraction(input: &str) -> Result<(usize, usize), &'static str> {
    input
        .split_once('/')
        .and_then(|(a, b)| {
            let a = a.parse::<usize>().ok()?;
            let b = b.parse::<usize>().ok()?;
            Some((a, b))
        })
        .ok_or("parse fraction error")
}
