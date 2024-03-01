use std::ops::Rem;

use crate::utils::parse_fraction;

use super::*;

#[derive(Debug, Copy, Clone)]
pub enum LengthUnit {
    Fraction(u32, u32),
    Unit(f32, &'static str),
}

impl Display for LengthUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fraction(a, b) => write!(f, "{}/{}", a, b),
            Self::Unit(a, b) => write!(f, "{}{}", *a as usize, b),
        }
    }
}

impl LengthUnit {
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/length#syntax>
    pub fn parse_fraction(input: &str) -> Result<Self> {
        let (a, b) = parse_fraction(input)?;
        Ok(Self::radio(a as u32, b as u32))
    }
    pub fn parse_length(input: &str) -> Result<Self> {
        Self::parse_unit(input, Self::parse_css_length_unit)
    }
    pub fn parse_angle(input: &str) -> Result<Self> {
        Self::parse_unit(input, Self::parse_angle_unit)
    }

    fn parse_unit(
        input: &str,
        parse_unit: impl Fn(&str) -> Option<&'static str> + 'static,
    ) -> Result<Self> {
        let mut chars = input.chars().peekable();
        let mut last_valid_index = None;
        for (i, _) in input.char_indices() {
            match chars.peek() {
                Some(&ch) if ch.is_numeric() || ch == '.' || (i == 0 && ch == '-') => {
                    chars.next(); // consume the character
                    last_valid_index = Some(i);
                }
                _ => break,
            }
        }

        last_valid_index
            .and_then(|idx| {
                let (number, rest) = input.split_at(idx + 1);
                let number = number.parse::<f32>().unwrap();

                let unit = parse_unit(rest)?;

                Some(Self::Unit(number, unit))
            })
            .ok_or(TailwindError::syntax_error("Unknown LengthUnit"))
    }

    fn parse_angle_unit(input: &str) -> Option<&'static str> {
        match input {
            "deg" => Some("deg"),
            "grad" => Some("grad"),
            "rad" => Some("rad"),
            "turn" => Some("turn"),
            _ => None,
        }
    }

    fn parse_css_length_unit(input: &str) -> Option<&'static str> {
        match input {
            "px" => Some("px"),
            "cm" => Some("cm"),
            "mm" => Some("mm"),
            "Q" => Some("Q"),
            "in" => Some("in"),
            "pc" => Some("pc"),
            "pt" => Some("pt"),
            "em" => Some("em"),
            "rem" => Some("rem"),
            "vw" => Some("vw"),
            "vh" => Some("vh"),
            "vmin" => Some("vmin"),
            "vmax" => Some("vmax"),
            "ex" => Some("ex"),
            "ch" => Some("ch"),
            "lh" => Some("lh"),
            "%" => Some("%"),
            _ => None,
        }
    }

    pub fn px(x: f32) -> Self {
        Self::Unit(x, "px")
    }
    pub fn em(x: f32) -> Self {
        Self::Unit(x, "em")
    }
    pub fn rem(x: f32) -> Self {
        Self::Unit(x, "rem")
    }
    pub fn percent(x: f32) -> Self {
        Self::Unit(x, "%")
    }
    pub fn radio(a: u32, b: u32) -> Self {
        if b.eq(&0) {
            return Self::Fraction(0, 1);
        }
        let n = gcd(a, b);
        Self::Fraction(a / n, b / n)
    }
}

pub fn gcd<T>(a: T, b: T) -> T
where
    T: PartialEq + Rem<Output = T> + Default + Copy,
{
    if b == T::default() {
        a
    } else {
        gcd(b, a % b)
    }
}

impl LengthUnit {
    #[inline]
    pub fn get_class(&self) -> String {
        self.to_string()
    }
    #[inline]
    pub fn get_class_arbitrary(&self) -> String {
        format!("[{}]", self)
    }

    pub fn is_fraction(&self) -> bool {
        matches!(self, Self::Fraction { .. })
    }
    pub fn is_fraction_eq(&self) -> bool {
        match self {
            Self::Fraction(a, b) => a.eq(b),
            _ => false,
        }
    }
    pub fn is_fraction_zero(&self) -> bool {
        match self {
            Self::Fraction(a, _) => a.eq(&0),
            _ => false,
        }
    }
}
