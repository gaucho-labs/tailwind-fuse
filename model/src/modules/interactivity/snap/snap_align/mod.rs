#[derive(Debug, Clone)]
pub struct TailwindSnapAlign {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindSnapAlign => "scroll-snap-align", [
    "start","end", "center", "align-none"
]);
