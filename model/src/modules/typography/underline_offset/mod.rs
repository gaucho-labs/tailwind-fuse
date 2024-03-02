use super::*;

#[derive(Debug, Clone)]
pub struct TailwindUnderlineOffset {}

impl TailwindInstance for TailwindUnderlineOffset {
    fn collision_id(&self) -> &'static str {
        "underline-offset".into()
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}
