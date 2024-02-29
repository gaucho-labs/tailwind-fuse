use super::*;

impl Display for SizingUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Min => write!(f, "min"),
            Self::Max => write!(f, "max"),
            Self::Fit => write!(f, "fit"),
            Self::Auto => write!(f, "auto"),
            Self::Full => write!(f, "full"),
            Self::Screen => write!(f, "screen"),
            Self::Fraction(numerator, denominator) => write!(f, "{}/{}", numerator, denominator),
            Self::Length(x) => write!(f, "[{}]", x),
        }
    }
}

impl SizingUnit {}

impl TailwindSizingKind {
    fn is_width(self) -> bool {
        matches!(self, Self::Width | Self::MinWidth | Self::MaxWidth)
    }
}

impl Display for TailwindSizingKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Width => f.write_str("width"),
            Self::MinWidth => f.write_str("min-width"),
            Self::MaxWidth => f.write_str("max-width"),
            Self::Height => f.write_str("height"),
            Self::MinHeight => f.write_str("min-height"),
            Self::MaxHeight => f.write_str("max-height"),
        }
    }
}

impl Display for TailwindSizing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.kind, self.size)
    }
}

impl TailwindInstance for TailwindSizing {
    fn collision_id(&self) -> String {
        match self.kind {
            TailwindSizingKind::Width => "width".to_string(),
            TailwindSizingKind::MinWidth => "min-width".to_string(),
            TailwindSizingKind::MaxWidth => "max-width".to_string(),
            TailwindSizingKind::Height => "height".to_string(),
            TailwindSizingKind::MinHeight => "min-height".to_string(),
            TailwindSizingKind::MaxHeight => "max-height".to_string(),
        }
    }

    fn get_collisions(&self) -> Vec<String> {
        vec![self.collision_id()]
    }
}
