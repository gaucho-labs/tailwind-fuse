#[derive(Debug, Clone)]
pub struct TailwindTextOverflow {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindTextOverflow => "text-overflow", ["truncate, ellipsis, clip"]);
