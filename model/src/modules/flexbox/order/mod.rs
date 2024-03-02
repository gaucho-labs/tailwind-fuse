use super::*;

/// <https://tailwindcss.com/docs/order
/// https://developer.mozilla.org/en-US/docs/Web/CSS/object-fit#syntax
#[derive(Debug, Clone)]
pub struct TailWindOrder {}

impl TailwindInstance for TailWindOrder {
    fn collision_id(&self) -> &'static str {
        "order"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}
