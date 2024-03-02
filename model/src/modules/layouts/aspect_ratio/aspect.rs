use super::*;

#[derive(Clone, Debug)]
pub enum Aspect {
    Radio(usize, usize),
    Standard(String),
    Arbitrary,
}

impl Aspect {
    pub fn parse(pattern: &[&str], _: &TailwindArbitrary) -> Result<Self> {
        let out = match pattern {
            ["square"] => Self::Radio(1, 1),
            ["video"] => Self::Radio(16, 9),
            [s] if Self::check_valid(s) => Self::Standard(s.to_string()),
            [n] => {
                let (a, b) = TailwindArbitrary::from(*n).as_fraction()?;
                Self::Radio(a, b)
            }
            [] => Self::Arbitrary,
            _ => return syntax_error!("unknown aspect-ratio elements"),
        };
        Ok(out)
    }
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["auto", "inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
}
