/// <https://tailwindcss.com/docs/align-self>
/// <https://developer.mozilla.org/en-US/docs/Web/CSS/align-self#syntax>
#[derive(Clone, Debug)]
pub struct TailwindSelf {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindSelf => "align-self", ["auto", "start", "end", "center", "stretch", "baseline"]);
