use super::*;

#[derive(Clone, Debug)]
pub struct TailwindBoxDecoration {
    kind: StandardValue,
}

crate::macros::keyword_instance!(TailwindBoxDecoration => "box-decoration-break");

impl TailwindBoxDecoration {
    /// <https://tailwindcss.com/docs/box-decoration-break>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            kind: StandardValue::parser("box-break", &Self::check_valid)(pattern, arbitrary)?,
        })
    }
    /// dispatch to [box-decoration-break](https://developer.mozilla.org/en-US/docs/Web/CSS/box-decoration-break)
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        StandardValue::parse_arbitrary(arbitrary).map(|kind| Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/box-decoration-break#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "clone", "inherit", "initial", "revert", "slice", "unset",
        ]);
        set.contains(mode)
    }
}
