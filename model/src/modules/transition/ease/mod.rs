use crate::TailwindInstance;

#[derive(Clone, Debug)]
pub struct TailwindEase {}

impl TailwindInstance for TailwindEase {
    fn collision_id(&self) -> &'static str {
        "transition-timing-function"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}
