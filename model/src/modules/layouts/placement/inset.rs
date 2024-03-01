use super::*;

#[derive(Clone, Debug)]
pub struct TailwindInset {
    axis: AxisXY,
    kind: UnitValue,
}

impl Display for TailwindInset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.write_negative(f)?;
        self.axis.write_xyn(f, "inset", &self.kind)
    }
}

// TODO: HOW DOES INSET INTERACT WITH TOP/BOTTOM/LEFT/RIGHT?
impl TailwindInstance for TailwindInset {
    fn collision_id(&self) -> String {
        self.axis.collision_id("inset")
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

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
