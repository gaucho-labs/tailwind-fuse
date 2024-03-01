use super::*;

#[derive(Clone, Debug)]
pub struct TailwindShadowColor {
    color: TailwindColor,
}

crate::macros::color_instance!(TailwindShadowColor);

impl Display for TailwindShadowColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "shadow-{}", self.color)
    }
}

impl TailwindInstance for TailwindShadowColor {
    fn collision_id(&self) -> String {
        "shadow-color".into()
    }

    fn get_collisions(&self) -> Vec<String> {
        vec![self.collision_id()]
    }
}
