use super::*;

#[derive(Clone, Debug)]
pub struct TailwindZIndex {}

impl TailwindInstance for TailwindZIndex {
    fn collision_id(&self) -> &'static str {
        "z-index"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindZIndex {}
