use super::*;

#[derive(Clone, Debug)]
pub struct TailwindBackgroundColor {
    color: TailwindColor,
}
crate::macros::color_instance!(TailwindBackgroundColor);

impl Display for TailwindBackgroundColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "bg-{}", self.color)
    }
}

impl TailwindInstance for TailwindBackgroundColor {
    fn collision_id(&self) -> String {
        "bg-".to_string()
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}
