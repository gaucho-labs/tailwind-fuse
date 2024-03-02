use super::*;

#[derive(Debug, Clone)]
pub struct TailwindAlign {
    kind: VerticalAlign,
}

#[derive(Debug, Clone)]
pub enum VerticalAlign {
    Standard(String),
    Length(LengthUnit),
}

impl<T> From<T> for TailwindAlign
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self {
            kind: VerticalAlign::Standard(kind.into()),
        }
    }
}

// TODO: CHECK THIS
impl TailwindInstance for TailwindAlign {
    fn collision_id(&self) -> &'static str {
        "vertical-align"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindAlign {
    /// https://tailwindcss.com/docs/text-align
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            [s] if Self::check_valid(s) => Ok(Self {
                kind: VerticalAlign::Standard(s.to_string()),
            }),
            [s] => {
                let n = TailwindArbitrary::from(*s).as_length_or_fraction()?;
                Ok(Self {
                    kind: VerticalAlign::Length(n),
                })
            }
            [] => Self::parse_arbitrary(arbitrary),
            _ => syntax_error!("Unknown align instructions: {}", pattern.join("-")),
        }
    }
    /// https://tailwindcss.com/docs/text-align
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            kind: VerticalAlign::Length(arbitrary.as_length_or_fraction()?),
        })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/vertical-align#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "baseline",
            "sub",
            "super",
            "text-top",
            "text-bottom",
            "middle",
            "top",
            "bottom",
            "inherit",
            "initial",
            "revert",
            "unset",
        ]);
        set.contains(mode)
    }
}
