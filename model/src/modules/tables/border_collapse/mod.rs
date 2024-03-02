#[derive(Clone, Debug)]
pub struct TailwindBorderCollapse {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindBorderCollapse => "border-collapse", ["collapse", "separate"]);
