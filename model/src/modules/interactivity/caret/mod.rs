use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindCaretColor {
    color: TailwindColor,
}

crate::macros::color_instance!(TailwindCaretColor);

impl Display for TailwindCaretColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "caret-{}", self.color)
    }
}

impl TailwindInstance for TailwindCaretColor {
    fn collision_id(&self) -> String {
        "caret-color".into()
    }

    fn get_collisions(&self) -> Vec<String> {
        vec![self.collision_id()]
    }
}
