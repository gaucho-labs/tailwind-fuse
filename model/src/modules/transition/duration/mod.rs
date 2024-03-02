use super::*;

#[derive(Clone, Debug)]
pub struct TailwindDuration {
    ms: NumericValue,
}

impl TailwindInstance for TailwindDuration {
    fn collision_id(&self) -> &'static str {
        "transition-duration".into()
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindDuration {
    /// <https://tailwindcss.com/docs/transition-duration>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let ms = match pattern {
            [] if arbitrary.is_none() => 150u32.into(),
            _ => NumericValue::positive_parser("duration", Self::check_valid)(pattern, arbitrary)?,
        };
        Ok(Self { ms })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/transition-delay#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
}
