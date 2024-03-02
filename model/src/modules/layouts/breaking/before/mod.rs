use super::*;

#[derive(Clone, Debug)]
pub struct TailwindBreakBefore {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindBreakBefore => "break-before", [
    "auto",
    "avoid",
    "all",
    "avoid-page",
    "page",
    "left",
    "right",
    "column",
]);
