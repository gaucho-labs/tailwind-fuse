use super::*;

#[derive(Debug, Clone)]
pub struct TailwindJustifyContent {
    kind: StandardValue,
}

crate::macros::keyword_instance!(TailwindJustifyContent => "justify-content");

impl TailwindJustifyContent {
    /// <https://tailwindcss.com/docs/justify-content>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            kind: StandardValue::parser("justify-content", &Self::check_valid)(pattern, arbitrary)?,
        })
    }
    /// dispatch to [justify-content](https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content)
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        StandardValue::parse_arbitrary(arbitrary).map(|kind| Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "center",
            "end",
            "flex-end",
            "flex-start",
            "inherit",
            "initial",
            "left",
            "normal",
            "revert",
            "right",
            "space-around",
            "space-between",
            "space-evenly",
            "start",
            "stretch",
            "unset",
        ]);
        set.contains(mode)
    }
}
