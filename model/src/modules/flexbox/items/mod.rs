/// <https://tailwindcss.com/docs/align-items>
/// <https://developer.mozilla.org/en-US/docs/Web/CSS/align-items#syntax>
#[derive(Clone, Debug)]
pub struct TailwindItems {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindItems => "align-items", [
    "start", "end", "center","baseline", "stretch"
]);
