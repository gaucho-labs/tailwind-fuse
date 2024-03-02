use super::*;

#[derive(Debug, Clone)]
pub struct TailwindGridFlow {
    kind: StandardValue,
}

crate::macros::keyword_instance!(TailwindGridFlow => "grid-auto-flow");

impl TailwindGridFlow {
    /// <https://tailwindcss.com/docs/grid-auto-flow>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            ["col"] => StandardValue::from("column"),
            ["col", "dense"] => StandardValue::from("column dense"),
            ["row", "dense"] => StandardValue::from("row dense"),
            _ => StandardValue::parser("grid-auto", &Self::check_valid)(pattern, arbitrary)?,
        };
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-flow#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "column", "dense", "inherit", "initial", "revert", "row", "unset",
        ]);
        set.contains(mode)
    }
}
