use super::*;

#[derive(Debug, Clone)]
enum FontSmoothing {
    Antialias,
    Subpixel,
    Standard(String),
    Length(LengthUnit),
}

#[derive(Debug, Clone)]
pub struct TailwindFontSmoothing {
    kind: FontSmoothing,
}

impl<T> From<T> for TailwindFontSmoothing
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self {
            kind: FontSmoothing::Standard(kind.into()),
        }
    }
}

impl TailwindInstance for TailwindFontSmoothing {
    fn collision_id(&self) -> &'static str {
        "font-smoothing".into()
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindFontSmoothing {
    /// https://tailwindcss.com/docs/font-smoothing
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            ["antialiased"] => FontSmoothing::Antialias,
            ["subpixel", "antialiased"] | ["subpixel"] => FontSmoothing::Subpixel,
            [n] if Self::check_valid(n) => FontSmoothing::Standard(n.to_string()),
            [n] => {
                let l = TailwindArbitrary::from(*n).as_length_or_fraction()?;
                FontSmoothing::Length(l)
            }
            [] => FontSmoothing::Length(arbitrary.as_length_or_fraction()?),
            _ => {
                return syntax_error!("Unknown font-smoothing instructions: {}", pattern.join("-"))
            }
        };
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/font-smooth#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "auto", "never", "always", "inherit", "initial", "revert", "unset",
        ]);
        set.contains(mode)
    }
}
