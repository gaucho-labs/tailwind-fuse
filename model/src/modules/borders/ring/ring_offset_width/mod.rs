use super::*;

#[derive(Clone, Debug)]
pub struct TailwindRingOffsetWidth {
    kind: NumericValue,
}

impl TailwindInstance for TailwindRingOffsetWidth {
    fn collision_id(&self) -> &'static str {
        "ring-offset-width"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindRingOffsetWidth {
    /// <https://tailwindcss.com/docs/ring-width>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            [] => NumericValue::from(3u32),
            _ => NumericValue::positive_parser("ring-offset-width", Self::check_valid)(
                pattern, arbitrary,
            )?,
        };
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/object-fit#syntax
    pub fn check_valid(mode: &str) -> bool {
        ["inherit", "initial", "revert", "unset"].contains(&mode)
    }
}
