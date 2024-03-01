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
    fn collision_id(&self) -> String {
        match &self.kind {
            StandardValue::Keyword(s) => match s.as_str() {
                "normal-nums" => "fvn-normal".into(),
                "ordinal" => "fvn-ordinal".into(),
                "slashed-zero" => "fvn-slashed-zero".into(),
                "lining-nums" | "oldstyle-nums" => "fvn-figure".into(),
                "proportional-nums" | "tabular-nums" => "fvn-spacing".into(),
                "diagonal-fractions" | "stacked-fractions" => "fvn-fraction".into(),
                _ => s.clone(),
            },
            StandardValue::Arbitrary(_) => self.to_string(),
        }
    }

    fn get_collisions(&self) -> Vec<String> {
        match &self.kind {
            StandardValue::Keyword(s) if s == "normal-nums" => {
                let collisions = [
                    "fvn-normal",
                    "fvn-ordinal",
                    "fvn-slashed-zero",
                    "fvn-figure",
                    "fvn-spacing",
                    "fvn-fraction",
                ];
                collisions.iter().map(|s| s.to_string()).collect()
            }
            _ => vec!["fvn-normal".into(), self.collision_id()],
        }
    }
}

// crate::macros::keyword_instance!(TailwindFontVariantNumeric => "font-variant-numeric");

impl Display for TailwindFontVariantNumeric {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            StandardValue::Keyword(s) => match s.as_str() {
                "normal" => write!(f, "normal-nums"),
                "ordinal" | "slashed-zero" | "lining-nums" | "oldstyle-nums"
                | "proportional-nums" | "tabular-nums" | "diagonal-fractions"
                | "stacked-fractions" => write!(f, "{}", s),
                _ => write!(f, "font-numeric-{}", s),
            },
            StandardValue::Arbitrary(s) => write!(f, "font-numeric-{}", s.get_class()),
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
