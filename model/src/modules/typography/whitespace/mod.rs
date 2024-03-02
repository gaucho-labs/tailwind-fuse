#[derive(Debug, Clone)]
pub struct TailwindWhiteSpace {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindWhiteSpace => "white-space", [
    "normal",
    "nowrap",
    "pre",
    "pre-line",
    "pre-wrap",
    "break-spaces",
]);
