use super::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct TailwindRingInset {}

impl Display for TailwindRingInset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ring-inset")
    }
}

impl TailwindInstance for TailwindRingInset {
    fn collision_id(&self) -> String {
        "ring-inset".to_string()
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}
