use super::*;

#[derive(Debug, Clone)]
pub struct TailwindFontStyle {
    kind: StandardValue,
}

crate::macros::keyword_instance!(TailwindFontStyle => "font-style");

impl TailwindFontStyle {
    /// <https://tailwindcss.com/docs/font-style>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            kind: StandardValue::parser("font-style", &Self::check_valid)(pattern, arbitrary)?,
        })
    }
    /// dispatch to [font-style](https://developer.mozilla.org/en-US/docs/Web/CSS/font-style)
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        StandardValue::parse_arbitrary(arbitrary).map(|kind| Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/font-style#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "inherit", "initial", "italic", "normal", "oblique", "revert", "unset",
        ]);
        set.contains(mode)
    }
}
