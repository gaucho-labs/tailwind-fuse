use super::*;

#[derive(Clone, Debug)]
pub struct TailwindBreakBefore {
    kind: StandardValue,
}

crate::macros::keyword_instance!(TailwindBreakBefore => "break-before");

impl TailwindBreakBefore {
    /// <https://tailwindcss.com/docs/break-before>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            kind: StandardValue::parser("break-before", &Self::check_valid)(pattern, arbitrary)?,
        })
    }

    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        StandardValue::parse_arbitrary(arbitrary).map(|kind| Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/break-before#syntax
    pub fn check_valid(mode: &str) -> bool {
        TailwindBreakAfter::check_valid(mode)
    }
}
