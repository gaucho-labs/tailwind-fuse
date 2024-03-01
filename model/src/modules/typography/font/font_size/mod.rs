use super::*;

#[derive(Debug, Clone)]
pub struct TailwindFontSize {
    name: String,
}

impl Display for TailwindFontSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "text-{}", self.name)
    }
}

impl TailwindInstance for TailwindFontSize {
    fn collision_id(&self) -> String {
        "font-size".into()
    }

    fn get_collisions(&self) -> Vec<String> {
        vec![self.collision_id()]
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
