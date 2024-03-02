use super::*;

#[derive(Clone, Debug)]
pub struct TailwindClear {
    kind: &'static str,
}

// <https://tailwindcss.com/docs/clear>
// <https://developer.mozilla.org/en-US/docs/Web/CSS/clear#syntax>

crate::macros::keyword_instance!(TailwindClear => "clear", [
    "both",
    "inherit",
    "initial",
    "inline-end",
    "inline-start",
    "left",
    "none",
    "revert",
    "right",
    "unset",
]);
