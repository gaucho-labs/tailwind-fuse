use super::*;

#[derive(Clone, Debug)]
pub struct TailwindOutlineWidth {
    kind: UnitValue,
}

impl<T> From<T> for TailwindOutlineWidth
where
    T: Into<UnitValue>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl TailwindInstance for TailwindOutlineWidth {
    fn collision_id(&self) -> &'static str {
        "outline-width".into()
    }
    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindOutlineWidth {
    /// <https://tailwindcss.com/docs/outline-width>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind =
            UnitValue::positive_parser("outline-width", Self::check_valid, true, false, false)(
                pattern, arbitrary,
            )?;
        Ok(Self { kind })
    }
    ///
    pub fn check_valid(mode: &str) -> bool {
        [
            "inherit", "initial", "medium", "revert", "thick", "thin", "unset",
        ]
        .contains(&mode)
    }
}
