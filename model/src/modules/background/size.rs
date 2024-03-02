use super::*;

// TODO: How can you resolve conflicts with custom bg-attributes?
#[derive(Debug, Clone)]
pub struct TailwindBackgroundSize {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindBackgroundSize => "background-size", ["auto", "cover", "contain"]);
