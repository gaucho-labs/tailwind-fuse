#[derive(Debug, Clone)]
pub struct TailwindDecorationStyle {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindDecorationStyle => "text-decoration-style", ["solid", "double", "dotted", "dashed", "wavy"]);
