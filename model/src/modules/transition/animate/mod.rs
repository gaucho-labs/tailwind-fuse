use super::*;

use self::animation::Animation;

mod animation;

#[derive(Clone, Debug)]
pub struct TailwindAnimate {
    kind: Animation,
}

// TODO: Can animations be stacked??
impl TailwindInstance for TailwindAnimate {
    fn collision_id(&self) -> &'static str {
        "animation"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindAnimate {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            kind: Animation::parse(pattern, arbitrary)?,
        })
    }
}
