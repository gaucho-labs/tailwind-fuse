use super::*;

#[derive(Clone, Debug)]
pub struct TailwindBackgroundAttachment {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindBackgroundAttachment => "background-attachment", ["fixed", "local", "scroll"]);
