use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindDelay {
    ms: NumericValue,
}

impl Display for TailwindDelay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "delay-{}", self.ms)
    }
}

impl TailwindInstance for TailwindDelay {
    fn collision_id(&self) -> String {
        "transition-delay".into()
    }

    fn get_collisions(&self) -> Vec<String> {
        vec![self.collision_id()]
    }
}

impl TailwindDelay {
    /// <https://tailwindcss.com/docs/transition-delay>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let ms = match pattern {
            [] if arbitrary.is_none() => 100u32.into(),
            _ => NumericValue::positive_parser("delay", Self::check_valid)(pattern, arbitrary)?,
        };
        Ok(Self { ms })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/transition-delay#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
}
