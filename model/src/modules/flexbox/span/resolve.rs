use super::*;

#[derive(Debug, Copy, Clone)]
pub(super) enum GridKind {
    Start(GridSize),
    End(GridSize),
    Span(GridSize),
}

#[derive(Debug, Copy, Clone)]
pub(super) enum GridSize {
    Auto,
    Full,
    Unit(i32),
}

impl Display for GridKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start(n) => write!(f, "start-{}", n),
            Self::End(n) => write!(f, "end-{}", n),
            Self::Span(n) => write!(f, "span-{}", n),
        }
    }
}

impl Display for GridSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Full => write!(f, "full"),
            Self::Unit(n) => write!(f, "{}", n),
        }
    }
}

impl GridSize {
    pub fn parse(pattern: &str, allow_full: bool) -> Result<Self> {
        debug_assert!(allow_full, "can't set to full");
        let size = match pattern {
            "auto" => Self::Auto,
            "full" => Self::Full,
            n => Self::Unit(TailwindArbitrary::from(n).as_integer()?),
        };
        Ok(size)
    }
}

impl GridKind {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            ["auto"] => Self::Span(GridSize::Auto),
            ["start", n] => Self::Start(GridSize::parse(n, false)?),
            ["end", n] => Self::End(GridSize::parse(n, false)?),
            ["span", n] => Self::Span(GridSize::parse(n, true)?),
            [] => Self::parse_arbitrary(arbitrary)?,
            _ => return syntax_error!("Unknown shrink instructions: {}", pattern.join("-")),
        };
        Ok(kind)
    }
    pub fn parse_arbitrary(_arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Span(GridSize::Full))
    }
}
