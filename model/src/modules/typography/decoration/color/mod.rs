use super::*;

#[derive(Debug, Clone)]
pub struct TailwindDecorationColor {
    color: TailwindColor,
}

crate::macros::color_instance!(TailwindDecorationColor);

impl Display for TailwindDecorationColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "decoration-{}", self.color.get_class())
    }
}

impl TailwindInstance for TailwindDecorationColor {
    fn collision_id(&self) -> String {
        "decoration-color".into()
    }

    fn get_collisions(&self) -> Vec<String> {
        vec![self.collision_id()]
    }
}
