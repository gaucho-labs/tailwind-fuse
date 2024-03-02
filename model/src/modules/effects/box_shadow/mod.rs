use super::*;

/// https://tailwindcss.com/docs/box-shadow
#[derive(Clone, Debug)]
pub struct TailwindShadow {
    kind: StandardValue,
    drop: Backdrop,
}

impl TailwindInstance for TailwindShadow {
    fn collision_id(&self) -> &'static str {
        "box-shadow"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindShadow {
    /// <https://tailwindcss.com/docs/box-shadow>
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary, drop: bool) -> Result<Self> {
        let kind = match input {
            [] if arbitrary.is_some() => StandardValue::parse_arbitrary(arbitrary)?,
            _ => StandardValue::Keyword(input.join("-")),
        };
        Ok(Self {
            kind,
            drop: Backdrop(drop),
        })
    }
    /// <https://tailwindcss.com/docs/box-shadow#arbitrary-values>
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary, drop: bool) -> Result<Self> {
        Ok(Self {
            kind: StandardValue::parse_arbitrary(arbitrary)?,
            drop: Backdrop(drop),
        })
    }
}
