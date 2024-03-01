use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindStrokeColor {
    color: TailwindColor,
}

crate::macros::color_instance!(TailwindStrokeColor);

impl Display for TailwindStrokeColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "stroke-{}", self.color)
    }
}

impl TailwindInstance for TailwindStrokeColor {
    fn collision_id(&self) -> String {
        "stroke-color".into()
    }

    fn get_collisions(&self) -> Vec<String> {
        vec![self.collision_id()]
    }
}
