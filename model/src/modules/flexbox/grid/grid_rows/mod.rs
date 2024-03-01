use super::*;

#[derive(Debug, Clone)]
pub struct TailwindGridRows {
    kind: GridTemplate,
}

impl Display for TailwindGridRows {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "grid-rows-{}", self.kind)
    }
}

impl TailwindInstance for TailwindGridRows {
    fn collision_id(&self) -> String {
        "grid-rows".into()
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindGridRows {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            kind: GridTemplate::parse(pattern, arbitrary)?,
        })
    }
}
