use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBackgroundImage {
    kind: AnchorPoint,
}

impl Display for TailwindBackgroundImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "bg-gradient-{}", self.kind.get_class())
    }
}

impl TailwindInstance for TailwindBackgroundImage {
    fn collision_id(&self) -> String {
        todo!()
    }

    fn get_collisions(&self) -> Vec<String> {
        todo!()
    }
}

impl TailwindBackgroundImage {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            kind: AnchorPoint::parse(pattern, arbitrary, false)?,
        })
    }
}
