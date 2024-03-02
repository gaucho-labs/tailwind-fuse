use super::*;

#[derive(Debug, Clone)]
pub struct TailwindBoxSizing {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindBoxSizing => "box-sizing", ["border", "content"]);
