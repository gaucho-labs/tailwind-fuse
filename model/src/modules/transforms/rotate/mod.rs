use super::*;

#[derive(Clone, Debug)]
pub struct TailwindRotate {
    kind: UnitValue,
}

impl TailwindInstance for TailwindRotate {
    fn collision_id(&self) -> &'static str {
        "rotate"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
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
