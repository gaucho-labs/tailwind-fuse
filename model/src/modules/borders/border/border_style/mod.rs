#[derive(Clone, Debug)]
pub struct TailwindBorderStyle {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindBorderStyle => "border-style", ["solid", "dashed", "dotted", "double", "hidden", "none"]);
