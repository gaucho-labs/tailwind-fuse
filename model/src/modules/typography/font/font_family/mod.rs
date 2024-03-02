use crate::TailwindInstance;

#[derive(Debug, Clone)]
pub struct TailwindFontFamily {}

impl TailwindInstance for TailwindFontFamily {
    fn collision_id(&self) -> &'static str {
        "font-family"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}
