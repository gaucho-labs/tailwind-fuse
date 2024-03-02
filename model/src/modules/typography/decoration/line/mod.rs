#[derive(Debug, Clone)]
pub struct TailwindDecorationLine {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindDecorationLine => "text-decoration-line", [
    "underline", "overline", "line-through", "no-underline"
]);
