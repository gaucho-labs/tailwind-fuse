use super::*;
use crate::NumericValue;

#[derive(Debug, Clone)]
pub struct TailWindOrder {
    kind: NumericValue,
}

impl TailwindInstance for TailWindOrder {
    fn collision_id(&self) -> &'static str {
        "order"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailWindOrder {
    pub fn parse(
        pattern: &[&str],
        arbitrary: &TailwindArbitrary,
        negative: Negative,
    ) -> Result<Self> {
        let kind = match pattern {
            ["none"] => NumericValue::from(0i32),
            ["first"] => NumericValue::from(9999i32),
            ["last"] => NumericValue::from(-9999i32),
            [s] if Self::check_valid(s) => NumericValue::Keyword(s.to_string()),
            _ => NumericValue::negative_parser("order", &Self::check_valid)(
                pattern, arbitrary, negative,
            )?,
        };
        Ok(Self { kind })
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            kind: NumericValue::parse_arbitrary(arbitrary)?,
        })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/object-fit#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
}
