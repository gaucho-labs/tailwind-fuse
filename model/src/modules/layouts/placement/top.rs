use super::*;

/// <https://tailwindcss.com/docs/top-right-bottom-left>
/// <https://developer.mozilla.org/en-US/docs/Web/CSS/top#syntax>
#[derive(Clone, Debug)]
pub struct TailwindTop {}

// TODO: HOW DO THESE REFINE?
impl TailwindInstance for TailwindTop {
    fn collision_id(&self) -> &'static str {
        "top"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}
