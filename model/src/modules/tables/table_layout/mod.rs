#[derive(Debug, Clone)]
pub struct TailwindTableLayout {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindTableLayout => "table-layout", ["auto", "fixed"]);
