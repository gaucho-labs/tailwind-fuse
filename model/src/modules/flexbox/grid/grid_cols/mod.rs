use super::*;

#[derive(Debug, Clone)]
pub struct TailwindGridColumns {
    kind: GridTemplate,
}

impl TailwindInstance for TailwindGridColumns {
    fn collision_id(&self) -> &'static str {
        "grid-columns"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindGridColumns {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            kind: GridTemplate::parse(pattern, arbitrary)?,
        })
    }
}
