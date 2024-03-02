use super::*;

/// <https://tailwindcss.com/docs/flex-shrink>
/// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-shrink#syntax>
#[derive(Debug, Clone)]
pub struct TailWindShrink {}

impl TailwindInstance for TailWindShrink {
    fn collision_id(&self) -> &'static str {
        "shrink"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}
