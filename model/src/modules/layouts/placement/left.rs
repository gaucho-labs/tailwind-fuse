use super::*;

/// <https://tailwindcss.com/docs/top-right-bottom-left>
/// <https://developer.mozilla.org/en-US/docs/Web/CSS/left#syntax>
#[derive(Clone, Debug)]
pub struct TailwindLeft {}

// TODO CONFIRM
impl TailwindInstance for TailwindLeft {
    fn collision_id(&self) -> &'static str {
        "left"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}
