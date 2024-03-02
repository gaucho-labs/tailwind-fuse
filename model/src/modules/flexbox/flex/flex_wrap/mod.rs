#[derive(Debug, Clone)]
pub struct TailwindFlexWrap {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindFlexWrap => "flex-wrap", [
    "wrap",
    "wrap-reverse",
    "nowrap",
]);
