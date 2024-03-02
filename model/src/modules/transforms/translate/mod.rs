use super::*;

#[derive(Clone, Debug)]
pub struct TailwindTranslate {
    axis: AxisXY,
    kind: UnitValue,
}

crate::macros::axisxy_collision!(TailwindTranslate => "translate");

impl TailwindTranslate {
    /// <https://tailwindcss.com/docs/translate>
    pub fn parse(
        pattern: &[&str],
        arbitrary: &TailwindArbitrary,
        negative: Negative,
    ) -> Result<Self> {
        let (axis, rest) = AxisXY::split_xyn(pattern);
        let kind = match rest {
            ["px"] => UnitValue::px(1.0),
            ["full"] => UnitValue::radio(1, 1),
            _ => UnitValue::negative_parser("translate", |_| false, true, false, false)(
                rest, arbitrary, negative,
            )?,
        };
        Ok(Self { kind, axis })
    }
}
