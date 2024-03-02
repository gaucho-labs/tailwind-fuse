use super::*;

#[derive(Clone, Debug)]
pub struct TailwindObjectFit {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindObjectFit => "object-fit", [
    "contain", "cover", "fill", "none", "scale-down"
]);
