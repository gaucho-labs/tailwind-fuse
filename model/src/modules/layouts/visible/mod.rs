use super::*;

/// <https://tailwindcss.com/docs/visibility>
/// https://developer.mozilla.org/en-US/docs/Web/CSS/visibility#syntax
#[derive(Debug, Clone)]
pub struct TailwindVisibility {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindVisibility => "visibility", [
    "visible", "invisible", "collapse"
]);
