use super::*;

#[derive(Debug, Clone)]
pub struct TailwindBackgroundOrigin {
    kind: StandardValue,
}

crate::macros::keyword_instance!(TailwindBackgroundOrigin => "background-origin");

impl TailwindBackgroundOrigin {
    /// https://tailwindcss.com/docs/background-origin
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            ["border"] => StandardValue::from("border-box"),
            ["padding"] => StandardValue::from("padding-box"),
            ["content"] => StandardValue::from("content-box"),
            _ => StandardValue::parser("bg-origin", &Self::check_valid)(pattern, arbitrary)?,
        };
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/background-origin#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "border-box",
            "content-box",
            "inherit",
            "initial",
            "padding-box",
            "revert",
            "unset",
        ]);
        set.contains(mode)
    }
}
