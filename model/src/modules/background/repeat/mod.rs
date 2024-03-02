use super::*;

#[derive(Debug, Clone)]
pub struct TailwindBackgroundRepeat {
    kind: StandardValue,
}

crate::macros::keyword_instance!(TailwindBackgroundRepeat => "background-repeat");

impl TailwindBackgroundRepeat {
    /// <https://tailwindcss.com/docs/background-repeat>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            [] if arbitrary.is_none() => StandardValue::from("repeat"),
            ["none"] => StandardValue::from("no-repeat"),
            ["x"] => StandardValue::from("repeat-x"),
            ["y"] => StandardValue::from("repeat-y"),
            _ => StandardValue::parser("bg-repeat", &Self::check_valid)(pattern, arbitrary)?,
        };
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/background-repeat#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "inherit",
            "initial",
            "no-repeat",
            "repeat",
            "repeat-x",
            "repeat-y",
            "revert",
            "round",
            "space",
            "unset",
        ]);
        set.contains(mode)
    }
}
