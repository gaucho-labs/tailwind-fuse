use crate::StandardValue;

use super::*;

#[derive(Clone, Debug)]
pub struct TailwindOverscroll {
    kind: StandardValue,
    axis: AxisXY,
}

crate::axisxy_collision!(TailwindOverscroll => "overscroll");

impl TailwindOverscroll {
    /// <https://tailwindcss.com/docs/overscroll-behavior>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let (axis, rest) = AxisXY::split_xyn(pattern);
        let kind = StandardValue::parser("overscroll", &Self::check_valid)(rest, arbitrary)?;
        Ok(Self { kind, axis })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/overscroll-behavior#syntax
    pub fn check_valid(mode: &str) -> bool {
        [
            "auto", "contain", "inherit", "initial", "none", "revert", "unset",
        ]
        .contains(&mode)
    }
}
