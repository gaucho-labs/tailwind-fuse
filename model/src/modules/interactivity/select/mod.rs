#[derive(Debug, Clone)]
pub struct TailwindSelect {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindSelect => "user-select", ["none", "text", "all", "auto"]);
