use super::*;

/// <https://tailwindcss.com/docs/flex-basis>
/// https://developer.mozilla.org/en-US/docs/Web/CSS/flex-basis#syntax
#[derive(Debug, Clone)]
pub struct TailwindBasis {}

impl TailwindInstance for TailwindBasis {
    fn collision_id(&self) -> &'static str {
        "basis"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}
