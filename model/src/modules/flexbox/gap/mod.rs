use super::*;
use crate::AxisXY;

#[derive(Debug, Copy, Clone)]
pub struct TailwindGap {
    size: LengthUnit,
    axis: AxisXY,
}

crate::axisxy_collision!(TailwindGap => "gap");

impl TailwindGap {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            ["x", rest @ ..] => Ok(Self {
                size: parse_size(rest, arbitrary)?,
                axis: AxisXY::X,
            }),
            ["y", rest @ ..] => Ok(Self {
                size: parse_size(rest, arbitrary)?,
                axis: AxisXY::Y,
            }),
            _ => Ok(Self {
                size: parse_size(pattern, arbitrary)?,
                axis: AxisXY::N,
            }),
        }
    }
}

fn parse_size(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<LengthUnit> {
    let size = match pattern {
        [] => arbitrary.as_length_or_fraction()?,
        ["px"] => LengthUnit::px(1.0),
        [n] => {
            let a = TailwindArbitrary::from(*n);
            LengthUnit::rem(a.as_float()? / 4.0)
        }
        _ => return syntax_error!("Unknown gap instructions"),
    };
    Ok(size)
}
