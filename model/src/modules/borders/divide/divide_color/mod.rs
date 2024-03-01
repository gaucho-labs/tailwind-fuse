use super::*;

#[derive(Clone, Debug)]
pub struct TailwindDivideColor {
    color: TailwindColor,
}

crate::macros::color_instance!(TailwindDivideColor);

impl Display for TailwindDivideColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ring-{}", self.color)
    }
}

impl TailwindInstance for TailwindDivideColor {
    fn collision_id(&self) -> String {
        "ring-".to_string()
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}
