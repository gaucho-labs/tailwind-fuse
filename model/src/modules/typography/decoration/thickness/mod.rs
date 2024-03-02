use crate::NumericValue;

use super::*;

#[derive(Debug, Clone)]
pub struct TailwindDecorationThickness {
    px: NumericValue,
}

impl<T> From<T> for TailwindDecorationThickness
where
    T: Into<NumericValue>,
{
    fn from(value: T) -> Self {
        Self { px: value.into() }
    }
}

impl TailwindInstance for TailwindDecorationThickness {
    fn collision_id(&self) -> &'static str {
        "decoration-thickness"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindDecorationThickness {
    /// <https://tailwindcss.com/docs/text-decoration-thickness>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = NumericValue::positive_parser("decoration-thick", Self::check_valid)(
            pattern, arbitrary,
        )?;
        Ok(Self { px: kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/text-decoration-thickness#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "auto",
            "from-font",
            "inherit",
            "initial",
            "revert",
            "unset",
        ]);
        set.contains(mode)
    }
}
