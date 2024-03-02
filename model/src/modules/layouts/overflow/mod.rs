use crate::AxisXY;

use super::*;

/// <https://tailwindcss.com/docs/overflow>
/// <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow#syntax>
#[derive(Clone, Debug)]
pub struct TailwindOverflow {
    axis: AxisXY,
}

crate::axisxy_collision!(TailwindOverflow => "overflow");

impl TailwindOverflow {
    pub fn parse(pattern: &[&str]) -> Self {
        let (axis, _) = AxisXY::split_xyn(pattern);
        Self { axis }
    }
}
