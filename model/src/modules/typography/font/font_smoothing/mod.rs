use super::*;

#[derive(Debug, Clone)]
enum FontSmoothing {
    Antialias,
    Subpixel,
    Standard(String),
    Length(LengthUnit),
}

#[doc=include_str!("readme.md")]
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

impl Display for TailwindFontSmoothing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            FontSmoothing::Antialias => write!(f, "antialiased"),
            FontSmoothing::Subpixel => write!(f, "subpixel-antialiased"),
            FontSmoothing::Standard(s) => write!(f, "font-smoothing-{}", s),
            FontSmoothing::Length(s) => write!(f, "font-smoothing-[{}]", s),
        }
    }
}

impl TailwindInstance for TailwindFontSmoothing {
    fn collision_id(&self) -> String {
        "font-smoothing".into()
    }

    fn get_collisions(&self) -> Vec<String> {
        vec![self.collision_id()]
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
