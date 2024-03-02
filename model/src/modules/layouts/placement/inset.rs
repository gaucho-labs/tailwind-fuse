use super::*;

#[derive(Clone, Debug)]
pub struct TailwindInset {
    axis: AxisXY,
    kind: UnitValue,
}

// TODO: HOW DOES INSET INTERACT WITH TOP/BOTTOM/LEFT/RIGHT?
crate::axisxy_collision!(TailwindInset => "inset");

impl TailwindInset {
    pub fn parse(
        pattern: &[&str],
        arbitrary: &TailwindArbitrary,
        negative: Negative,
    ) -> Result<Self> {
        let (axis, rest) = AxisXY::split_xyn(pattern);
        let kind = get_kind_px_full_auto_fact("inset", rest, arbitrary, negative)?;
        Ok(Self { axis, kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/inset#syntax>
    pub fn check_valid(mode: &str) -> bool {
        ["auto", "inherit", "initial", "revert", "unset"].contains(&mode)
    }
}
