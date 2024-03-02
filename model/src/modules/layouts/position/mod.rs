use super::*;

/// <https://tailwindcss.com/docs/position>
/// <https://developer.mozilla.org/en-US/docs/Web/CSS/position#syntax>
#[derive(Clone, Debug)]
pub struct TailwindPosition {
    kind: &'static str,
}
crate::macros::keyword_instance!(TailwindPosition => "position", [
    "static", "relative", "absolute", "fixed", "sticky", "inherit", "initial", "revert",
    "unset",
]);
