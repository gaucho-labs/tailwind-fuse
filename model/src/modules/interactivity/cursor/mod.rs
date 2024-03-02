use super::*;

#[derive(Debug, Clone)]
pub struct TailwindCursor {}

impl TailwindInstance for TailwindCursor {
    fn collision_id(&self) -> &'static str {
        "cursor"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}
