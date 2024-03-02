#[derive(Debug, Clone)]
pub struct TailwindScrollBehavior {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindScrollBehavior => "scroll-behavior", ["auto", "smooth"]);
