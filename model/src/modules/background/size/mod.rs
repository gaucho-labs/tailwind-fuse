use super::*;

#[derive(Debug, Clone)]
pub struct TailwindBackgroundSize {
    kind: StandardValue,
}

crate::macros::keyword_instance!(TailwindBackgroundSize => "background-size");

impl TailwindBackgroundSize {
    /// <https://tailwindcss.com/docs/background-size>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("bg-size", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/background-size#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "auto", "contain", "cover", "inherit", "initial", "revert", "unset",
        ]);
        set.contains(mode)
    }
}
