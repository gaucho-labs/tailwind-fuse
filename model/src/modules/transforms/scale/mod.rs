use super::*;

#[derive(Clone, Debug)]
pub struct TailwindScale {
    kind: NumericValue,
    axis: AxisXY,
}

crate::macros::axisxy_collision!(TailwindScale => "scale");

impl TailwindScale {
    // https://tailwindcss.com/docs/scale
    pub fn parse(
        pattern: &[&str],
        arbitrary: &TailwindArbitrary,
        negative: Negative,
    ) -> Result<Self> {
        let (axis, rest) = AxisXY::split_xyn(pattern);
        let kind = NumericValue::negative_parser("scale", |_| false)(rest, arbitrary, negative)?;
        Ok(TailwindScale { kind, axis })
    }
}
