use super::*;
mod cols;
pub use self::cols::Columns;

#[derive(Clone, Debug)]
pub struct TailwindColumns {
    kind: Columns,
}

impl TailwindInstance for TailwindColumns {
    fn collision_id(&self) -> &'static str {
        "columns"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindColumns {
    /// https://tailwindcss.com/docs/columns
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            kind: Columns::parse(input, arbitrary)?,
        })
    }
    /// dispatch to [columns](https://developer.mozilla.org/en-US/docs/Web/CSS/columns)
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            kind: Columns::parse_arbitrary(arbitrary)?,
        })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/columns#syntax>
    pub fn check_valid(mode: &str) -> bool {
        Columns::check_valid(mode)
    }
}
