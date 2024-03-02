use super::*;

#[derive(Debug, Clone)]
pub struct TailWindGrow {}

impl TailwindInstance for TailWindGrow {
    fn collision_id(&self) -> &'static str {
        "flexbox-grow"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailWindGrow {}
