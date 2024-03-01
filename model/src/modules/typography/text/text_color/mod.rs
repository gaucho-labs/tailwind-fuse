use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindTextColor {
    color: TailwindColor,
}

crate::macros::color_instance!(TailwindTextColor);

impl Display for TailwindTextColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "text-{}", self.color)
    }
}

impl TailwindInstance for TailwindTextColor {
    fn collision_id(&self) -> String {
        "text-color".into()
    }

    fn get_collisions(&self) -> Vec<String> {
        vec![self.collision_id()]
    }
}
