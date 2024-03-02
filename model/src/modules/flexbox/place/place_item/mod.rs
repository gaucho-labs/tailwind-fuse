#[derive(Debug, Clone)]
pub struct TailwindPlaceItems {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindPlaceItems => "place-items", [
    "start",
    "end",
    "center",
    "baseline",
    "stretch"
]);
