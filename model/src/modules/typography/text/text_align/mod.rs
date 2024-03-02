#[derive(Debug, Clone)]
pub struct TailwindTextAlignment {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindTextAlignment => "text-align", ["left", "center", "right", "justify", "start", "end"]);
