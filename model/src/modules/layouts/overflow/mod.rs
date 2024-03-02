use crate::{AxisXY, StandardValue};

use super::*;

#[derive(Clone, Debug)]
pub struct TailwindOverflow {
    kind: StandardValue,
    axis: AxisXY,
}

crate::axisxy_collision!(TailwindOverflow => "overflow");

impl TailwindOverflow {
    /// <https://tailwindcss.com/docs/overflow>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let (axis, rest) = AxisXY::split_xyn(pattern);
        let kind = StandardValue::parser("overflow", &Self::check_valid)(rest, arbitrary)?;
        Ok(Self { kind, axis })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow#syntax>
    pub fn check_valid(mode: &str) -> bool {
        [
            "auto", "clip", "hidden", "inherit", "initial", "revert", "scroll", "unset", "visible",
        ]
        .contains(&mode)
    }
}
