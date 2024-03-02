use super::*;

#[derive(Clone, Debug)]
pub struct TailwindOutlineOffset {
    kind: UnitValue,
}

impl TailwindInstance for TailwindOutlineOffset {
    fn collision_id(&self) -> &'static str {
        "outline-offset"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
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
