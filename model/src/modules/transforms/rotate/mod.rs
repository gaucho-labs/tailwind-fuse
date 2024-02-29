use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindRotate {
    kind: UnitValue,
}

impl Display for TailwindRotate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.write_negative(f)?;
        self.kind.write_class(f, "rotate-")
    }
}

impl TailwindInstance for TailwindRotate {
    fn collision_id(&self) -> String {
        "rotate".into()
    }

    fn get_collisions(&self) -> Vec<String> {
        vec![self.collision_id()]
    }
}

impl TailwindRotate {
    // <https://tailwindcss.com/docs/rotate>
    pub fn parse(
        input: &[&str],
        arbitrary: &TailwindArbitrary,
        negative: Negative,
    ) -> Result<Self> {
        let kind = UnitValue::negative_parser("scale", |_| false, false, false, false)(
            input, arbitrary, negative,
        )?;
        Ok(Self { kind })
    }
}
