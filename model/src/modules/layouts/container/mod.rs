use super::*;

#[derive(Copy, Clone, Debug, Default)]
pub struct TailwindContainer {}

impl TailwindInstance for TailwindContainer {
    fn collision_id(&self) -> &'static str {
        "container"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}
