use super::*;

#[derive(Clone, Debug)]
pub struct TailwindSnapStop {
    kind: StandardValue,
}

crate::macros::keyword_instance!(TailwindSnapStop => "scroll-snap-stop");

impl TailwindSnapStop {
    /// <https://tailwindcss.com/docs/scroll-snap-stop>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("scroll-stop", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/scroll-snap-stop#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["always", "inherit", "initial", "normal", "unset"]);
        set.contains(mode)
    }
}
