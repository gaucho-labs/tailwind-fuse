#[derive(Debug, Clone)]
pub struct TailwindAppearance {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindAppearance => "appearance", ["none", "auto"]);
