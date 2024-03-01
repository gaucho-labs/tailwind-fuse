use super::*;

#[derive(Clone, Debug)]
pub struct TailwindBorderColor {
    color: TailwindColor,
}

crate::macros::color_instance!(TailwindBorderColor);

impl Display for TailwindBorderColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "border-{}", self.color)
    }
}

impl TailwindInstance for TailwindBorderColor {
    fn collision_id(&self) -> String {
        "border-".to_string()
    }

    fn get_collisions(&self) -> Vec<String> {
        vec![self.collision_id()]
    }
}
