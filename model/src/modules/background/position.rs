use super::*;

#[derive(Clone, Debug)]
pub struct TailwindBackgroundPosition {
    kind: AnchorPoint,
}

impl TailwindInstance for TailwindBackgroundPosition {
    fn collision_id(&self) -> &'static str {
        "origin"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindBackgroundPosition {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            kind: AnchorPoint::parse(pattern, arbitrary, true)?,
        })
    }
}
