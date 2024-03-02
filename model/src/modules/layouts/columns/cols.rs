use super::*;

#[derive(Clone, Debug)]
pub enum Columns {
    Columns(i32),
    Length(LengthUnit),
    Standard(String),
    Arbitrary,
}

impl Columns {
    #[inline]
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let rem = |n: usize| Self::Length(LengthUnit::rem(n as f32));
        let out = match input {
            [s] if Self::check_valid(s) => Self::Standard(s.to_string()),
            ["3xs"] => rem(16),
            ["2xs"] => rem(18),
            ["xs"] => rem(20),
            ["sm"] => rem(24),
            ["md"] => rem(28),
            ["lg"] => rem(32),
            ["xl"] => rem(36),
            ["2xl"] => rem(42),
            ["3xl"] => rem(48),
            ["4xl"] => rem(56),
            ["5xl"] => rem(64),
            ["6xl"] => rem(72),
            ["7xl"] => rem(80),
            [name] => {
                let a = TailwindArbitrary::from(*name);
                Self::Columns(a.as_integer()?)
            }
            [] => Self::Arbitrary,
            _ => return syntax_error!("Unknown columns instructions: {}", input.join("-")),
        };
        Ok(out)
    }
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["auto", "inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
}
