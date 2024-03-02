use super::*;

#[derive(Debug, Clone)]
pub struct TailwindFlexDirection {
    kind: StandardValue,
}

crate::macros::keyword_instance!(TailwindFlexDirection => "flex-direction");

impl TailwindFlexDirection {
    /// <https://tailwindcss.com/docs/flex-direction>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("flex-direction", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-direction#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "column",
            "column-reverse",
            "inherit",
            "initial",
            "revert",
            "row",
            "row-reverse",
            "unset",
        ]);
        set.contains(mode)
    }
}
