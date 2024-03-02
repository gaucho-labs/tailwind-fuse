use super::*;

#[derive(Debug, Clone)]
pub struct TailwindPlaceContent {
    kind: StandardValue,
}

crate::macros::keyword_instance!(TailwindPlaceContent => "place-content");

impl TailwindPlaceContent {
    /// <https://tailwindcss.com/docs/place-content>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            kind: StandardValue::parser("place-content", &Self::check_valid)(pattern, arbitrary)?,
        })
    }
    /// dispatch to [place-items](https://developer.mozilla.org/en-US/docs/Web/CSS/place-content)
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        StandardValue::parse_arbitrary(arbitrary).map(|kind| Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/place-content#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "center",
            "end",
            "flex-end",
            "flex-start",
            "inherit",
            "initial",
            "normal",
            "revert",
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
