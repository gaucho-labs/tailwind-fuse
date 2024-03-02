use crate::StandardValue;

use super::*;

#[derive(Debug, Clone)]
pub struct TailwindFontVariantNumeric {
    kind: StandardValue,
}

impl<T> From<T> for TailwindFontVariantNumeric
where
    T: Into<String>,
{
    fn from(input: T) -> Self {
        Self {
            kind: StandardValue::from(input.into()),
        }
    }
}

impl TailwindInstance for TailwindFontVariantNumeric {
    fn collision_id(&self) -> &'static str {
        match &self.kind {
            StandardValue::Keyword(s) => match s.as_str() {
                "normal-nums" => "fvn-normal",
                "ordinal" => "fvn-ordinal",
                "slashed-zero" => "fvn-slashed-zero",
                "lining-nums" | "oldstyle-nums" => "fvn-figure",
                "proportional-nums" | "tabular-nums" => "fvn-spacing",
                "diagonal-fractions" | "stacked-fractions" => "fvn-fraction",
                _ => "fvn-other",
            },
            StandardValue::Arbitrary => "fvn-other",
        }
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        match &self.kind {
            StandardValue::Keyword(s) if s == "normal-nums" => vec![
                "fvn-normal",
                "fvn-ordinal",
                "fvn-slashed-zero",
                "fvn-figure",
                "fvn-spacing",
                "fvn-fraction",
            ],
            _ => vec!["fvn-normal"],
        }
    }
}

impl TailwindFontVariantNumeric {
    /// https://tailwindcss.com/docs/font-variant-numeric
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            kind: StandardValue::parser("font-numeric", &Self::check_valid)(pattern, arbitrary)?,
        })
    }
    /// dispatch to [font-variant-numeric](https://developer.mozilla.org/en-US/docs/Web/CSS/font-variant-numeric)
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        StandardValue::parse_arbitrary(arbitrary).map(|kind| Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/font-variant-numeric#syntax
    fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "diagonal-fractions",
            "inherit",
            "initial",
            "lining-nums",
            "normal",
            "oldstyle-nums",
            "ordinal",
            "proportional-nums",
            "slashed-zero",
            "stacked-fractions",
            "tabular-nums",
            "unset",
        ]);
        set.contains(mode)
    }
}
