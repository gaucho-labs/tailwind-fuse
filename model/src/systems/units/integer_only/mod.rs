use super::*;

mod traits;

/// Used to represent those attributes that only have integers
#[derive(Debug, Clone)]
pub enum NumericValue {
    Number {
        n: f32,
        negative: bool,
        can_be_negative: bool,
    },
    Keyword(String),
    Arbitrary,
}

impl NumericValue {
    pub fn negative_parser(
        id: &'static str,
        checker: impl Fn(&str) -> bool,
    ) -> impl Fn(&[&str], &TailwindArbitrary, Negative) -> Result<Self> {
        move |pattern: &[&str], _: &TailwindArbitrary, negative: Negative| {
            let joined = pattern.join("-");
            match pattern {
                _ if checker(&joined) => Ok(Self::Keyword(joined)),
                [] => Ok(Self::Arbitrary),
                [n] => Self::parse_number(n, negative),
                _ => Err(TailwindError::syntax_error(format!(
                    "Unknown {} pattern",
                    id
                ))),
            }
        }
    }
    pub fn positive_parser(
        id: &'static str,
        checker: impl Fn(&str) -> bool,
    ) -> impl Fn(&[&str], &TailwindArbitrary) -> Result<Self> {
        move |pattern: &[&str], _: &TailwindArbitrary| {
            let joined = pattern.join("-");
            match pattern {
                _ if checker(&joined) => Ok(Self::Keyword(joined)),
                [] => Ok(Self::Arbitrary),
                [n] => {
                    let i = TailwindArbitrary::from(*n).as_integer()?;
                    Ok(Self::Number {
                        n: i as f32,
                        negative: false,
                        can_be_negative: true,
                    })
                }
                _ => Err(TailwindError::syntax_error(format!(
                    "Unknown {} pattern",
                    id
                ))),
            }
        }
    }
    pub fn parse_number(n: &str, negative: Negative) -> Result<Self> {
        let mut n = TailwindArbitrary::from(n).as_float()?;
        if negative.0 {
            n = -n
        }
        Ok(Self::Number {
            n,
            negative: negative.0,
            can_be_negative: false,
        })
    }
}
