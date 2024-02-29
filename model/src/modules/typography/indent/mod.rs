use crate::UnitValue;

use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindIndent {
    kind: UnitValue,
}

impl Display for TailwindIndent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.write_class(f, "indent-")
    }
}

impl TailwindInstance for TailwindIndent {
    fn collision_id(&self) -> String {
        "indent".into()
    }

    fn get_collisions(&self) -> Vec<String> {
        vec![self.collision_id()]
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
