use super::*;

#[derive(Copy, Clone, Debug)]
pub enum SpacingAxis {
    All,
    X,
    Y,
    Top,
    Right,
    Bottom,
    Left,
}

impl Display for SpacingAxis {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let class = match self {
            SpacingAxis::All => "",
            SpacingAxis::X => "x",
            SpacingAxis::Y => "y",
            SpacingAxis::Top => "t",
            SpacingAxis::Right => "r",
            SpacingAxis::Bottom => "b",
            SpacingAxis::Left => "l",
        };
        write!(f, "{}", class)
    }
}

impl SpacingAxis {
    pub fn collision_id(&self, id: &'static str) -> String {
        format!("{}-{}", self, id)
    }

    pub fn collisions(&self, id: &'static str) -> Vec<String> {
        let collisions = match self {
            SpacingAxis::All => vec![
                SpacingAxis::All,
                SpacingAxis::X,
                SpacingAxis::Y,
                SpacingAxis::Top,
                SpacingAxis::Right,
                SpacingAxis::Bottom,
                SpacingAxis::Left,
            ],
            SpacingAxis::X => vec![SpacingAxis::X, SpacingAxis::Left, SpacingAxis::Right],
            SpacingAxis::Y => vec![SpacingAxis::Y, SpacingAxis::Top, SpacingAxis::Bottom],
            other => vec![*other],
        };

        collisions
            .into_iter()
            .map(|axis| axis.collision_id(id))
            .collect()
    }
}
