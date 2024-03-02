#[derive(Clone, Debug)]
pub struct TailwindContentAlign {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindContentAlign => "align-content", [
    "normal",
    "center",
    "flex-start",
    "flex-end",
    "space-between",
    "space-around",
    "space-evenly",
    "stretch",
    "baseline",
]);
