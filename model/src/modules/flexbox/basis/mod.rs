use super::*;

#[derive(Debug, Clone)]
enum Basis {
    Number(f32),
    Length(LengthUnit),
    // TODO: should probably be static?
    Standard(String),
    Arbitrary,
}

#[derive(Debug, Clone)]
pub struct TailwindBasis {
    kind: Basis,
}

// TODO: Is this flex basis? is there another basis?
impl TailwindInstance for TailwindBasis {
    fn collision_id(&self) -> &'static str {
        "basis"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindBasis {
    /// <https://tailwindcss.com/docs/flex-basis>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            kind: Basis::parse(pattern, arbitrary)?,
        })
    }

    /// https://developer.mozilla.org/en-US/docs/Web/CSS/flex-basis#syntax
    pub fn check_valid(mode: &str) -> bool {
        Basis::check_valid(mode)
    }
}

impl Basis {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let out = match pattern {
            ["px"] => Self::Length(LengthUnit::px(1.0)),
            ["full"] => Self::Length(LengthUnit::Fraction(1, 1)),
            ["fit" | "min" | "max", "content"] => Self::Standard(pattern.join("-")),
            [s @ ("fit" | "min" | "max")] => Self::Standard(format!("{}-content", s)),
            [s] if Self::check_valid(s) => Self::Standard(s.to_string()),
            [n] => {
                let a = TailwindArbitrary::new(*n);
                Self::maybe_length(&a).or_else(|_| Self::maybe_float(&a))?
            }
            [] => Self::Arbitrary,
            _ => return syntax_error!("Unknown basis instructions"),
        };
        Ok(out)
    }
    fn maybe_float(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Number(arbitrary.as_float()?))
    }
    fn maybe_length(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Length(arbitrary.as_length_or_fraction()?))
    }
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "auto",
            "content",
            "fill",
            "fit-content",
            "inherit",
            "initial",
            "max-content",
            "min-content",
            "revert",
            "unset",
        ]);
        set.contains(mode)
    }
}
