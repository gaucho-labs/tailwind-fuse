use super::*;

#[derive(Clone, Debug)]
pub struct TailwindOrigin {
    kind: AnchorPoint,
}

impl TailwindInstance for TailwindOrigin {
    fn collision_id(&self) -> &'static str {
        "transform-origin"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindOrigin {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            kind: AnchorPoint::parse(pattern, arbitrary, true)?,
        })
    }
}
