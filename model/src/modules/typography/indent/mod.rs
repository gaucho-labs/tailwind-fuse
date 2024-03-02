use crate::UnitValue;

use super::*;

#[derive(Debug, Clone)]
pub struct TailwindIndent {
    kind: UnitValue,
}

impl TailwindInstance for TailwindIndent {
    fn collision_id(&self) -> &'static str {
        "indent"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindIndent {
    /// <https://tailwindcss.com/docs/text-indent>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            ["px"] => UnitValue::px(1.0),
            _ => UnitValue::positive_parser("id", Self::check_valid, true, false, false)(
                pattern, arbitrary,
            )?,
        };
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/word-break#syntax
    pub fn check_valid(mode: &str) -> bool {
        ["inherit", "initial", "revert", "unset"].contains(&mode)
    }
}
