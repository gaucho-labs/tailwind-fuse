#[derive(Debug, Clone)]
pub struct TailwindPointerEvents {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindPointerEvents => "pointer-events", ["auto", "none"]);
