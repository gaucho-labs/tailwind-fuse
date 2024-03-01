use super::*;

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
        "bg-gradient-".to_string()
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
