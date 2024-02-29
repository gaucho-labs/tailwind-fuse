use super::*;

///
#[derive(Copy, Clone, Debug)]
pub struct SpacingAxis {
    pub class: &'static str,
    pub attributes: &'static [&'static str],
}

impl Display for SpacingAxis {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.class)
    }
}

impl SpacingAxis {
    pub fn new(class: &'static str, attributes: &'static [&'static str]) -> Self {
        Self { class, attributes }
    }
}
