use super::*;

#[derive(Debug, Clone)]
pub struct TailwindScrollBehavior {
    kind: StandardValue,
}

crate::macros::keyword_instance!(TailwindScrollBehavior => "scroll-behavior");

impl TailwindScrollBehavior {
    /// <https://tailwindcss.com/docs/scroll-behavior>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind =
            StandardValue::parser("scroll-behavior", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/scroll-behavior#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "auto", "inherit", "initial", "revert", "smooth", "unset",
        ]);
        set.contains(mode)
    }
}
