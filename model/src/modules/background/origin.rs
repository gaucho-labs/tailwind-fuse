use super::*;

#[derive(Debug, Clone)]
pub struct TailwindBackgroundOrigin {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindBackgroundOrigin => "background-origin", ["border", "padding", "content"]);
