#[derive(Debug, Clone)]
pub struct TailwindSnapType {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindSnapType => "scroll-snap-type", ["none", "x", "y", "both", "mandatory", "proximity"]);
