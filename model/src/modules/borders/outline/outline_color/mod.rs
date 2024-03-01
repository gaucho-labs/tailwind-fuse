use super::*;

#[derive(Clone, Debug)]
pub struct TailwindOutlineColor {
    color: TailwindColor,
}

crate::macros::color_instance!(TailwindOutlineColor);

impl Display for TailwindOutlineColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "outline-color-{}", self.color)
    }
}

impl TailwindInstance for TailwindOutlineColor {
    fn collision_id(&self) -> String {
        "outline-color".to_string()
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}
