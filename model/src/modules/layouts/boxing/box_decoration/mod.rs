use super::*;

#[derive(Clone, Debug)]
pub struct TailwindBoxDecoration {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindBoxDecoration => "box-decoration-break", ["clone", "slice"]);
