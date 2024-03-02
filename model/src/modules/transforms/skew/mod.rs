use super::*;

#[derive(Clone, Debug)]
pub struct TailwindSkew {
    axis: AxisXY,
    kind: UnitValue,
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
