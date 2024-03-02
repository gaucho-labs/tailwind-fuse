#[derive(Debug, Clone)]
pub struct TailwindJustifyContent {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindJustifyContent => "justify-content", [
    "normal",
    "start",
    "end",
    "center",
    "space-between",
    "space-around",
    "space-evenly"
]);
