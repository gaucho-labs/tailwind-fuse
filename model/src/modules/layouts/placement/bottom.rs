use super::*;

/// <https://tailwindcss.com/docs/top-right-bottom-left>
/// <https://developer.mozilla.org/en-US/docs/Web/CSS/bottom#syntax>
#[derive(Clone, Debug)]
pub struct TailwindBottom {}

// TODO: NOT SURE
impl TailwindInstance for TailwindBottom {
    fn collision_id(&self) -> &'static str {
        "bottom"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}
