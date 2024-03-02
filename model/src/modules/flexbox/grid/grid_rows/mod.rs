use super::*;

#[derive(Debug, Clone)]
pub struct TailwindGridRows {
    kind: GridTemplate,
}

impl TailwindInstance for TailwindGridRows {
    fn collision_id(&self) -> &'static str {
        "grid-rows"
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
