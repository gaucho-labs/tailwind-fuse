#[derive(Debug, Clone)]
pub struct TailwindPlaceSelf {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindPlaceSelf => "place-self", [
    "auto",
    "start",
    "end",
    "center",
    "stretch"
]);
