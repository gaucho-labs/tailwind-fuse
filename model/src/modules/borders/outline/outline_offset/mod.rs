use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindOutlineOffset {
    kind: UnitValue,
}

impl Display for TailwindOutlineOffset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "outline-offset-{}", self.kind)
    }
}

impl TailwindInstance for TailwindOutlineOffset {
    fn collision_id(&self) -> String {
        "outline-offset".into()
    }

    fn get_collisions(&self) -> Vec<String> {
        vec![self.collision_id()]
    }
}

impl TailwindOutlineOffset {
    /// <https://tailwindcss.com/docs/outline-offset>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind =
            UnitValue::positive_parser("outline-offset", Self::check_valid, true, true, false)(
                pattern, arbitrary,
            )?;
        Ok(Self { kind })
    }
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
}
