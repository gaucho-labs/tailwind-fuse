use super::*;

use self::animation::Animation;

mod animation;

#[derive(Clone, Debug)]
pub struct TailwindAnimate {
    kind: Animation,
}

impl Display for TailwindAnimate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "animate-{}", self.kind)
    }
}

// TODO: Can animations be stacked??
impl TailwindInstance for TailwindAnimate {
    fn collision_id(&self) -> String {
        "animation".into()
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
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            kind: Animation::parse_arbitrary(arbitrary)?,
        })
    }
}
