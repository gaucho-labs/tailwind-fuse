#[derive(Debug, Clone)]
pub struct TailwindJustifyItems {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindJustifyItems => "justify-items", ["start", "end", "center", "stretch"]);
