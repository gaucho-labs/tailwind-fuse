use super::*;

#[derive(Debug, Clone)]
pub struct TailwindStrokeWidth {
    kind: StrokeWidth,
}

#[derive(Debug, Clone)]
enum StrokeWidth {
    Unit(i32),
    Length(LengthUnit),
}

impl Display for StrokeWidth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StrokeWidth::Unit(s) => write!(f, "{}", s),
            StrokeWidth::Length(s) => write!(f, "{}", s.get_class()),
        }
    }
}

impl Display for TailwindStrokeWidth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "stroke-{}", self.kind)
    }
}

impl TailwindInstance for TailwindStrokeWidth {
    fn collision_id(&self) -> String {
        "stroke-width".into()
    }

    fn get_collisions(&self) -> Vec<String> {
        vec![self.collision_id()]
    }
}

/// https://tailwindcss.com/docs/stroke-width
impl TailwindStrokeWidth {
    pub fn from_width(width: &str) -> Result<Self> {
        Ok(Self {
            kind: StrokeWidth::parse(width)?,
        })
    }

    pub fn from_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            kind: StrokeWidth::parse_arbitrary(arbitrary)?,
        })
    }
}

impl StrokeWidth {
    // stroke width should only be a number.
    pub fn parse(width: &str) -> Result<Self> {
        let a = TailwindArbitrary::from(width);
        Self::maybe_no_unit(&a)
    }

    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Self::maybe_length(arbitrary)
    }

    fn maybe_no_unit(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Unit(arbitrary.as_integer()?))
    }

    fn maybe_length(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Length(arbitrary.as_length_or_fraction()?))
    }
}
