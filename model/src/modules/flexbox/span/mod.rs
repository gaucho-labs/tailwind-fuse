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

// TODO: REVISE THIS. IT'S DEFINITELY WRONG SOMEHOW
impl TailwindInstance for TailwindColumn {
    fn collision_id(&self) -> String {
        match self.kind {
            GridKind::Span(_) => "grid-cols-span".into(),
            GridKind::Start(_) => "grid-cols-start".into(),
            GridKind::End(_) => "grid-cols-end".into(),
        }
    }

    fn get_collisions(&self) -> Vec<String> {
        vec![self.collision_id()]
    }
}

// TODO: REVISE THIS. IT'S DEFINITELY WRONG SOMEHOW
impl TailwindInstance for TailwindRow {
    fn collision_id(&self) -> String {
        match self.kind {
            GridKind::Span(_) => "grid-row-span".into(),
            GridKind::Start(_) => "grid-row-start".into(),
            GridKind::End(_) => "grid-row-end".into(),
        }
    }

    fn get_collisions(&self) -> Vec<String> {
        vec![self.collision_id()]
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
