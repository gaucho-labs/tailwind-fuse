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
