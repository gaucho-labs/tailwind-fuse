use super::*;

use self::resolve::GridKind;

mod resolve;

#[derive(Debug, Copy, Clone)]
pub struct TailwindRow {
    kind: GridKind,
}

#[derive(Debug, Copy, Clone)]
pub struct TailwindColumn {
    kind: GridKind,
}

// TODO: REVISE THIS. IT'S DEFINITELY WRONG SOMEHOW
// TODO: HIGH PRIORITY
// Can they all just be combined? https://tailwindcss.com/docs/grid-column#starting-and-ending-lines
impl TailwindInstance for TailwindColumn {
    fn collision_id(&self) -> &'static str {
        match self.kind {
            GridKind::Span(_) => "grid-cols-span",
            GridKind::Start(_) => "grid-cols-start",
            GridKind::End(_) => "grid-cols-end",
        }
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

// TODO: REVISE THIS. IT'S DEFINITELY WRONG SOMEHOW
impl TailwindInstance for TailwindRow {
    fn collision_id(&self) -> &'static str {
        match self.kind {
            GridKind::Span(_) => "grid-row-span",
            GridKind::Start(_) => "grid-row-start",
            GridKind::End(_) => "grid-row-end",
        }
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindRow {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            kind: GridKind::parse(pattern, arbitrary)?,
        })
    }
}

impl TailwindColumn {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            kind: GridKind::parse(pattern, arbitrary)?,
        })
    }
}
