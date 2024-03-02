#[derive(Debug, Clone)]
pub struct TailwindFontStyle {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindFontStyle => "font-style", ["italic", "not-italic"]);
