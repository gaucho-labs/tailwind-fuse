use super::*;

// TODO: HOW DOES INSET INTERACT WITH TOP/BOTTOM/LEFT/RIGHT?
// <https://developer.mozilla.org/en-US/docs/Web/CSS/inset#syntax>
#[derive(Clone, Debug)]
pub struct TailwindInset {
    axis: AxisXY,
}

crate::axisxy_collision!(TailwindInset => "inset");

impl TailwindInset {
    pub fn parse(pattern: &[&str]) -> Result<Self> {
        let (axis, _) = AxisXY::split_xyn(pattern);
        Ok(Self { axis })
    }

    pub fn check_valid(mode: &str) -> bool {
        ["auto", "inherit", "initial", "revert", "unset"].contains(&mode)
    }
}
