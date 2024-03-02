#[derive(Debug, Clone)]
pub struct TailwindResize {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindResize => "resize", [
    "none",
    "x",
    "y",
]);
