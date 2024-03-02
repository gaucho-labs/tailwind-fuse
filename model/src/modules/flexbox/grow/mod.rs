use super::*;

#[derive(Debug, Clone)]
pub struct TailWindGrow {
    grow: NumericValue,
}

impl TailwindInstance for TailWindGrow {
    fn collision_id(&self) -> &'static str {
        "flexbox-grow"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailWindGrow {
    /// https://tailwindcss.com/docs/flex-grow
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let grow = match pattern {
            [] if arbitrary.is_none() => 100u32.into(),
            _ => NumericValue::positive_parser("grow", Self::check_valid)(pattern, arbitrary)?,
        };
        Ok(Self { grow })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-grow#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
}
