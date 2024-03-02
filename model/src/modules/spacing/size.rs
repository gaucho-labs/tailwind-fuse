use super::*;

#[derive(Debug, Clone)]
pub(super) enum SpacingSize {
    Unit(f32),
    Standard(String),
    Arbitrary(TailwindArbitrary),
}

impl SpacingSize {
    pub fn parse(
        pattern: &[&str],
        arbitrary: &TailwindArbitrary,
        check_valid: &'static impl Fn(&str) -> bool,
    ) -> Result<Self> {
        match pattern {
            [] => Self::parse_arbitrary(arbitrary),
            ["px"] => Ok(Self::Arbitrary(TailwindArbitrary::from("1px"))),
            [n] if check_valid(n) => Ok(Self::Standard(n.to_string())),
            [n] => Ok(Self::Unit(TailwindArbitrary::from(*n).as_float()?)),
            _ => syntax_error!("Unknown padding instructions: {}", pattern.join("-")),
        }
    }

    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Arbitrary(TailwindArbitrary::new(arbitrary)?))
    }
}

impl SpacingSize {}
