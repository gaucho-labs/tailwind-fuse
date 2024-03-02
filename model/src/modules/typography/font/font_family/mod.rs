use super::*;
use crate::StandardValue;

#[derive(Debug, Clone)]
pub struct TailwindFontFamily {
    kind: StandardValue,
}

crate::macros::keyword_instance!(TailwindFontFamily => "font-family");
