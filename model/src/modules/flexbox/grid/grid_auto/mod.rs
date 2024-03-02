use crate::{modules::flexbox::*, Axis2D};

#[derive(Debug, Clone)]
enum GridAutoKind {
    Auto,
    Min,
    Max,
    Fr,
    Arbitrary,
}

#[derive(Debug, Clone)]
pub struct TailwindGridAuto {
    kind: GridAutoKind,
    axis: Axis2D,
}

crate::axis2d_collision!(TailwindGridAuto => "grid-auto");

impl GridAutoKind {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            [] => Self::Arbitrary,
            ["auto"] => Self::Auto,
            ["min"] => Self::Min,
            ["max"] => Self::Max,
            ["fr"] => Self::Fr,
            _ => return syntax_error!("Unknown shadow instructions: {}", pattern.join("-")),
        };
        Ok(kind)
    }
}

impl TailwindGridAuto {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let (axis, rest) = match pattern {
            ["rows", rest @ ..] => (Axis2D::X, rest),
            ["cols", rest @ ..] => (Axis2D::Y, rest),
            _ => return syntax_error!("Unknown auto instructions: {}", pattern.join("-")),
        };
        let kind = GridAutoKind::parse(rest, arbitrary)?;
        Ok(Self { kind, axis })
    }
}
