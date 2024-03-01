use crate::*;

pub fn parse_fraction(input: &str) -> Result<(usize, usize)> {
    input
        .split_once('/')
        .and_then(|(a, b)| {
            let a = a.parse::<usize>().ok()?;
            let b = b.parse::<usize>().ok()?;
            Some((a, b))
        })
        .ok_or(TailwindError::syntax_error("parse fraction error"))
}
