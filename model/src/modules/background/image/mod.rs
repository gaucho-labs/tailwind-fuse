use super::*;

#[derive(Clone, Debug)]
pub struct TailwindBackgroundImage {
    kind: AnchorPoint,
}

impl TailwindInstance for TailwindBackgroundImage {
    fn collision_id(&self) -> &'static str {
        "background-gradient"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindBackgroundImage {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            kind: AnchorPoint::parse(pattern, arbitrary, false)?,
        })
    }
}
