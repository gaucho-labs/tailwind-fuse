use super::*;

#[derive(Debug, Clone)]
pub struct TailwindTextAlignment {
    kind: StandardValue,
}

crate::macros::keyword_instance!(TailwindTextAlignment => "text-align");

impl TailwindTextAlignment {
    /// <https://tailwindcss.com/docs/will-change>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("font-align", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// dispatch to [text-align](https://developer.mozilla.org/en-US/docs/Web/CSS/text-align)
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        StandardValue::parse_arbitrary(arbitrary).map(|kind| Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/text-align#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "start",
            "end",
            "left",
            "right",
            "center",
            "justify",
            "justify-all",
            "match-parent",
            "inherit",
            "initial",
            "revert",
            "unset",
        ]);
        set.contains(mode)
    }
}
