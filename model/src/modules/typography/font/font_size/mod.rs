use super::*;

#[derive(Debug, Clone)]
pub struct TailwindFontSize {
    name: String,
}

impl TailwindInstance for TailwindFontSize {
    fn collision_id(&self) -> &'static str {
        "font-size".into()
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindFontSize {
    #[inline]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
