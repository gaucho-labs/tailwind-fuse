use super::*;

/// <https://tailwindcss.com/docs/top-right-bottom-left>
/// <https://developer.mozilla.org/en-US/docs/Web/CSS/right#syntax>
#[derive(Clone, Debug)]
pub struct TailwindRight {}

impl TailwindInstance for TailwindRight {
    fn collision_id(&self) -> &'static str {
        "right"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}
