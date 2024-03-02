/// <https://tailwindcss.com/docs/list-style-position>
/// https://developer.mozilla.org/en-US/docs/Web/CSS/list-style-position#syntax
#[derive(Debug, Clone)]
pub struct TailwindListPosition {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindListPosition => "list-style-position", ["inside", "outside"]);
