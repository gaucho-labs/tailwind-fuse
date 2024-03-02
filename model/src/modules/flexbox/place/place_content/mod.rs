#[derive(Debug, Clone)]
pub struct TailwindPlaceContent {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindPlaceContent => "place-content", [
    "center",
    "start",
    "end",
    "between",
    "around",
    "evenly",
    "baseline",
    "stretch"
]);
