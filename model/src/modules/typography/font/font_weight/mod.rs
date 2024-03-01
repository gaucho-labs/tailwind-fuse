use super::*;

// #[doc = include_str!("font-weight.md")]
#[derive(Debug, Clone)]
pub struct TailwindFontWeight {
    weight: i32,
}

impl Display for TailwindFontWeight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let text = match self.weight {
            100 => "thin",
            _ => return write!(f, "font-[{}]", self.weight),
        };
        write!(f, "font-{}", text)
    }
}

impl TailwindInstance for TailwindFontWeight {
    fn collision_id(&self) -> String {
        "font-weight".into()
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindFontWeight {
    pub const THIN: Self = Self { weight: 100 };
    pub const EXTRA_LIGHT: Self = Self { weight: 200 };
    pub const LIGHT: Self = Self { weight: 300 };
    pub const NORMAL: Self = Self { weight: 400 };
    pub const MEDIUM: Self = Self { weight: 500 };
    pub const SEMI_BOLD: Self = Self { weight: 600 };
    pub const BOLD: Self = Self { weight: 700 };
    pub const EXTRA_BOLD: Self = Self { weight: 800 };
    pub const BLACK: Self = Self { weight: 900 };
    #[inline]
    pub fn new(weight: i32) -> Self {
        Self { weight }
    }
}
