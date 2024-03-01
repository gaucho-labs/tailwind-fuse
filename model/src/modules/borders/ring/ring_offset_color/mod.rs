use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindRingOffsetColor {
    color: TailwindColor,
}

crate::macros::color_instance!(TailwindRingOffsetColor);

impl Display for TailwindRingOffsetColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ring-{}", self.color)
    }
}

impl TailwindInstance for TailwindRingOffsetColor {
    fn collision_id(&self) -> String {
        "ring-offset-color".into()
    }

    fn get_collisions(&self) -> Vec<String> {
        vec![self.collision_id()]
    }
}
