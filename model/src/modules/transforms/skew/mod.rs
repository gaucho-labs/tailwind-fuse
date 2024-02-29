use super::*;

#[doc=include_str!("readme.md")]
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

impl TailwindInstance for TailwindSkew {
    fn collision_id(&self) -> String {
        self.axis.collision_id("skew")
    }

    fn get_collisions(&self) -> Vec<String> {
        self.axis.collisions("skew")
    }
}

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
