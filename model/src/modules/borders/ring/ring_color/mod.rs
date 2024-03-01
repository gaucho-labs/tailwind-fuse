use super::*;

#[derive(Clone, Debug)]
pub struct TailwindRingColor {
    color: TailwindColor,
}

crate::macros::color_instance!(TailwindRingColor);

impl Display for TailwindRingColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ring-{}", self.color)
    }
}

impl TailwindInstance for TailwindRingColor {
    fn collision_id(&self) -> String {
        "ring-color".into()
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}
