use super::*;

#[derive(Debug, Clone)]
pub struct TailwindDecorationLine {
    kind: StandardValue,
}

crate::macros::keyword_instance!(TailwindDecorationLine => "text-decoration-line");

impl TailwindDecorationLine {
    /// <https://tailwindcss.com/docs/cursor>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            ["through"] => StandardValue::from("line-through"),
            _ => StandardValue::parser("decoration-line", &Self::check_valid)(pattern, arbitrary)?,
        };
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "blink",
            "inherit",
            "initial",
            "line-through",
            "none",
            "overline",
            "underline",
            "unset",
        ]);
        set.contains(mode)
    }
}
