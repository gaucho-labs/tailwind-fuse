use super::*;

#[doc=include_str!("readme.md")]
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

impl Display for TailwindOutlineWidth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            UnitValue::Number { n, .. } => write!(f, "outline-{}", n),
            UnitValue::Length(s) => write!(f, "outline-{}", s),
            UnitValue::Keyword(s) => write!(f, "outline-width-{}", s),
            UnitValue::Arbitrary(s) => write!(f, "outline-width-{}", s),
        }
    }
}

impl TailwindInstance for TailwindOutlineWidth {
    fn collision_id(&self) -> String {
        "outline-width".into()
    }

    fn get_collisions(&self) -> Vec<String> {
        vec![self.collision_id()]
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
