#[derive(Debug, Clone)]
pub struct TailwindJustifySelf {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindJustifySelf => "justify-self", ["auto", "start", "end", "center", "stretch"]);
