#[derive(Debug, Clone)]
pub struct TailwindFlexDirection {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindFlexDirection => "flex-direction", [
    "row",
    "row-reverse",
    "column",
    "column-reverse",
]);
