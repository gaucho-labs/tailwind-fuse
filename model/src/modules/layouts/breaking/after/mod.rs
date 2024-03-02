use super::*;

#[derive(Clone, Debug)]
pub struct TailwindBreakAfter {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindBreakAfter => "break-after", [
    "auto",
    "avoid",
    "all",
    "avoid-page",
    "page",
    "left",
    "right",
    "column",
]);
