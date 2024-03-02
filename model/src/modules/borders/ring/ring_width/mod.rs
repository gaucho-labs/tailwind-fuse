use crate::NumericValue;

use super::*;

#[derive(Clone, Debug)]
pub struct TailwindRingWidth {
    kind: NumericValue,
}

impl TailwindInstance for TailwindRingWidth {
    fn collision_id(&self) -> &'static str {
        "ring-width"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}
impl TailwindRingWidth {
    /// <https://tailwindcss.com/docs/ring-width>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            [] => NumericValue::from(3u32),
            _ => NumericValue::positive_parser("blur", Self::check_valid)(pattern, arbitrary)?,
        };
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/object-fit#syntax
    pub fn check_valid(mode: &str) -> bool {
        ["inherit", "initial", "revert", "unset"].contains(&mode)
    }
}
