use super::*;

#[derive(Clone, Debug)]
pub struct TailwindWillChange {}

impl TailwindInstance for TailwindWillChange {
    fn collision_id(&self) -> &'static str {
        "will-change"
    }
    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}
