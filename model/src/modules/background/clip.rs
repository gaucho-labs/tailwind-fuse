use super::*;

#[derive(Clone, Debug)]
pub struct TailwindBackgroundClip {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindBackgroundClip => "background-clip", ["border", "padding", "content", "text"]);
