use super::*;

use self::resolve::GridKind;

mod resolve;

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindRow {
    kind: GridKind,
}

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindColumn {
    kind: GridKind,
}

impl Display for TailwindRow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "row-{}", self.kind)
    }
}

impl Display for TailwindColumn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "col-{}", self.kind)
    }
}

impl TailwindInstance for TailwindColumn {}

impl TailwindInstance for TailwindRow {}

impl TailwindRow {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: GridKind::parse(pattern, arbitrary)? })
    }
}

impl TailwindColumn {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: GridKind::parse(pattern, arbitrary)? })
    }
}
