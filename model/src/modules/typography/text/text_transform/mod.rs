#[derive(Debug, Clone)]
pub struct TailwindTextTransform {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindTextTransform => "text-transform", [
    "uppercase", "lowercase", "capitalize", "normal-case"
]);
