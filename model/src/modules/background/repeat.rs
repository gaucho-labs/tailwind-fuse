use super::*;

#[derive(Debug, Clone)]
pub struct TailwindBackgroundRepeat {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindBackgroundRepeat => "background-repeat", [
    "no-repeat",
    "repeat",
    "repeat-x",
    "repeat-y",
    "repeat-round",
    "repeat-space"
]);
