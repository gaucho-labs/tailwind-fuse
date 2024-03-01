use super::*;
use crate::StandardValue;

#[derive(Debug, Clone)]
pub struct TailwindFontFamily {
    kind: StandardValue,
}

crate::macros::keyword_instance!(TailwindFontFamily => "font-family");

impl Display for TailwindFontFamily {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "font-{}", self.kind)
    }
}
