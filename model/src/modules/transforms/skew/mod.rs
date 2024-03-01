use super::*;

#[derive(Clone, Debug)]
pub struct TailwindSkew {
    axis: AxisXY,
    kind: UnitValue,
}

impl Display for TailwindSkew {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.write_negative(f)?;
        self.axis.write_xyn(f, "skew-", &self.kind)
    }
}

crate::macros::axisxy_collision!(TailwindSkew => "skew");

impl TailwindSkew {
    // <https://tailwindcss.com/docs/skew>
    pub fn parse(
        pattern: &[&str],
        arbitrary: &TailwindArbitrary,
        negative: Negative,
    ) -> Result<Self> {
        let (axis, rest) = AxisXY::split_xyn(pattern);
        let kind = UnitValue::negative_parser("skew", |_| false, false, false, false)(
            rest, arbitrary, negative,
        )?;
        Ok(Self { kind, axis })
    }
}
