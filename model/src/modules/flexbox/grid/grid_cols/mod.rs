use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindGridColumns {
    kind: GridTemplate,
}

impl Display for TailwindGridColumns {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "grid-cols-{}", self.kind)
    }
}

impl TailwindInstance for TailwindGridColumns {
    fn collision_id(&self) -> String {
        "grid-columns".into()
    }

    fn get_collisions(&self) -> Vec<String> {
        vec![self.collision_id()]
    }
}

impl TailwindGridColumns {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            kind: GridTemplate::parse(pattern, arbitrary)?,
        })
    }
}
