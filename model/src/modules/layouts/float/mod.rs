use super::*;

#[derive(Clone, Debug)]
pub struct TailwindFloat {
    kind: &'static str,
}

// https://tailwindcss.com/docs/float
// https://developer.mozilla.org/en-US/docs/Web/CSS/float#syntax
crate::macros::keyword_instance!(TailwindFloat => "float", [
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
