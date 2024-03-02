use super::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct TailwindRingInset {}

impl TailwindInstance for TailwindRingInset {
    fn collision_id(&self) -> &'static str {
        "ring-inset"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}
