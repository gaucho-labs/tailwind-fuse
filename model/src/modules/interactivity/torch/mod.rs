#[derive(Debug, Clone)]
pub struct TailwindTorch {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindTorch => "user-select", [
    "auto",
    "none",
    "pan-x",
    "pan-left",
    "pan-right",
    "pan-y",
    "pan-up",
    "pan-down",
    "pinch-zoom",
    "manipulation",
]);
