use super::*;

#[derive(Clone, Debug)]
pub struct TailwindOrigin {
    kind: AnchorPoint,
}

impl Display for TailwindOrigin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "origin-{}", self.kind.get_class())
    }
}

impl TailwindInstance for TailwindOrigin {
    fn collision_id(&self) -> String {
        "transform-origin".into()
    }

    fn get_collisions(&self) -> Vec<String> {
        vec![self.collision_id()]
    }
}

impl TailwindOrigin {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            kind: AnchorPoint::parse(pattern, arbitrary, true)?,
        })
    }
}
