use super::*;

#[derive(Debug, Clone)]
pub struct TailwindResize {
    kind: StandardValue,
}

crate::macros::keyword_instance!(TailwindResize => "resize");

impl TailwindResize {
    /// https://tailwindcss.com/docs/user-select
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            [] if arbitrary.is_none() => StandardValue::from("both"),
            ["x"] => StandardValue::from("horizontal"),
            ["y"] => StandardValue::from("vertical"),
            _ => StandardValue::parser("resize", &Self::check_valid)(pattern, arbitrary)?,
        };
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/user-select#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "block",
            "both",
            "horizontal",
            "inherit",
            "initial",
            "inline",
            "none",
            "revert",
            "unset",
            "vertical",
        ]);
        set.contains(mode)
    }
}
