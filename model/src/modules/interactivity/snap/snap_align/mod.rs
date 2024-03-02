use super::*;

#[derive(Debug, Clone)]
pub struct TailwindSnapAlign {
    kind: StandardValue,
}

crate::macros::keyword_instance!(TailwindSnapAlign => "scroll-snap-align");

impl TailwindSnapAlign {
    /// <https://tailwindcss.com/docs/scroll-snap-align>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("scroll-align", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/scroll-snap-align#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "center", "inherit", "initial", "none", "revert", "unset",
        ]);
        set.contains(mode)
    }
}
