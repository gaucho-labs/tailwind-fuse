use super::*;

#[derive(Clone, Debug)]
pub(super) enum Animation {
    None,
    Spin,
    Ping,
    Pulse,
    Bounce,
    Arbitrary,
}

impl Animation {
    pub fn parse(pattern: &[&str], _: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            ["none"] => Self::None,
            ["spin"] => Self::Spin,
            ["ping"] => Self::Ping,
            ["pulse"] => Self::Pulse,
            ["bounce"] => Self::Bounce,
            [] => Self::Arbitrary,
            _ => return syntax_error!("Unknown outline instructions: {}", pattern.join("-")),
        };
        Ok(kind)
    }
}
