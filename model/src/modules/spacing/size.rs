use super::*;

#[derive(Debug, Clone)]
pub(super) enum SpacingSize {
    Unit(f32),
    Standard(String),
    Arbitrary,
}

impl SpacingSize {
    pub fn parse(
        pattern: &[&str],
        arbitrary: &TailwindArbitrary,
        check_valid: &'static impl Fn(&str) -> bool,
    ) -> Result<Self> {
        match pattern {
            [] => Ok(Self::Arbitrary),
            // ["px"] => Ok(Self::Arbitrary(TailwindArbitrary::from("1px"))),
            ["px"] => Ok(Self::Arbitrary),
            [n] if check_valid(n) => Ok(Self::Standard(n.to_string())),
            [n] => Ok(Self::Unit(TailwindArbitrary::from(*n).as_float()?)),
            _ => syntax_error!("Unknown padding instructions: {}", pattern.join("-")),
        }
    }
}

impl SpacingSize {}
