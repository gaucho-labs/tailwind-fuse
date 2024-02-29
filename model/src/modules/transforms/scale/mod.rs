use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindScale {
    kind: NumericValue,
    axis: AxisXY,
}
impl Display for TailwindScale {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.write_negative(f)?;
        match self.axis {
            AxisXY::N => write!(f, "scale-{}", self.kind),
            AxisXY::X => write!(f, "scale-x-{}", self.kind),
            AxisXY::Y => write!(f, "scale-y-{}", self.kind),
        }
    }
}

impl TailwindInstance for TailwindScale {
    fn collision_id(&self) -> String {
        self.axis.collision_id("scale")
    }

    fn get_collisions(&self) -> Vec<String> {
        self.axis.collisions("scale")
    }
}

// noinspection DuplicatedCode
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
