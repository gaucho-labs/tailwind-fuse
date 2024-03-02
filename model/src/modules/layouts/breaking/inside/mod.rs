use super::*;

#[derive(Clone, Debug)]
pub struct TailwindBreakInside {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindBreakInside => "break-inside", [
    "auto", "avoid", "avoid-page", "avoid-column"
]);
