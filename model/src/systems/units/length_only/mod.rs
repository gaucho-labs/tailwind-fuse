use super::*;

mod traits;

#[derive(Clone, Debug)]
pub enum UnitValue {
    Number { n: f32, is_negative: bool },
    Length(LengthUnit),
    Keyword(String),
    Arbitrary,
}

impl UnitValue {
    pub fn px(x: f32) -> Self {
        Self::Length(LengthUnit::px(x))
    }
    pub fn radio(a: u32, b: u32) -> Self {
        Self::Length(LengthUnit::radio(a, b))
    }
}

pub fn is_negative(value: &UnitValue) -> bool {
    match *value {
        UnitValue::Number { n, is_negative } if is_negative => n < 0.0,
        _ => false,
    }
}

impl UnitValue {
    pub fn negative_parser(
        id: &'static str,
        check_valid: impl Fn(&str) -> bool,
        is_length: bool,
        is_integer: bool,
        allow_fraction: bool,
    ) -> impl Fn(&[&str], &TailwindArbitrary, Negative) -> Result<Self> {
        move |pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative| {
            let kind = match pattern {
                [] => Self::parse_arbitrary(arbitrary)?,
                [s] if check_valid(s) => Self::Keyword(s.to_string()),
                [n] => Self::parse_number(n, negative, is_length, is_integer, allow_fraction)?,
                _ => {
                    let msg = format!("Unknown {} instructions: {}", id, pattern.join("-"));
                    return Err(TailwindError::syntax_error(msg));
                }
            };
            Ok(kind)
        }
    }
    pub fn positive_parser(
        id: &'static str,
        check_valid: impl Fn(&str) -> bool,
        is_length: bool,
        is_integer: bool,
        allow_fraction: bool,
    ) -> impl Fn(&[&str], &TailwindArbitrary) -> Result<Self> {
        move |pattern: &[&str], arbitrary: &TailwindArbitrary| {
            let kind = match pattern {
                [] => Self::parse_arbitrary(arbitrary)?,
                [s] if check_valid(s) => Self::Keyword(s.to_string()),
                [n] => Self::parse_number(
                    n,
                    Negative::from(true),
                    is_length,
                    is_integer,
                    allow_fraction,
                )?,
                _ => {
                    let msg = format!("Unknown {} instructions: {}", id, pattern.join("-"));
                    return Err(TailwindError::syntax_error(msg));
                }
            };
            Ok(kind)
        }
    }
    pub fn parse_arbitrary(_: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Arbitrary)
    }
    pub fn parse_number(
        n: &str,
        negative: Negative,
        is_length: bool,
        is_integer: bool,
        can_be_fraction: bool,
    ) -> Result<Self> {
        let a = TailwindArbitrary::from(n);
        match is_length {
            true => Self::maybe_length(&a, can_be_fraction),
            false => Self::maybe_angle(&a),
        }
        .or_else(|_| Self::maybe_number(&a, negative, is_integer))
    }
    fn maybe_number(
        arbitrary: &TailwindArbitrary,
        negative: Negative,
        is_integer: bool,
    ) -> Result<Self> {
        let mut n = match is_integer {
            true => arbitrary.as_integer()? as f32,
            false => arbitrary.as_float()?,
        };
        if negative.0 {
            n = -n
        };
        Ok(Self::Number {
            n,
            is_negative: negative.0,
        })
    }
    fn maybe_length(arbitrary: &TailwindArbitrary, allow_fraction: bool) -> Result<Self> {
        let n = match allow_fraction {
            true => arbitrary.as_length_or_fraction(),
            false => arbitrary.as_length(),
        }?;
        Ok(Self::Length(n))
    }
    fn maybe_angle(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Length(arbitrary.as_angle()?))
    }
}
