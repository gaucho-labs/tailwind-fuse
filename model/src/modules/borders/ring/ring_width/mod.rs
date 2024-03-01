use crate::NumericValue;

use super::*;

#[derive(Clone, Debug)]
pub struct TailwindRingWidth {
    kind: NumericValue,
}

impl Display for TailwindRingWidth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ring-offset-{}", self.kind)
    }
}

impl TailwindInstance for TailwindRingWidth {
    fn collision_id(&self) -> String {
        "ring-width".into()
    }

    fn get_collisions(&self) -> Vec<String> {
        vec![self.collision_id()]
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
