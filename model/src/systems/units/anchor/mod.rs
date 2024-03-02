use super::*;

/// Anchor points are used to position elements relative to the viewport.
#[derive(Clone, Debug)]
pub enum AnchorPoint {
    /// `0% 0%` to the viewport.
    LeftTop,
    /// `50% 0%` to the viewport.
    Top,
    /// `100% 0%` to the viewport.
    RightTop,
    /// `0% 50%` to the viewport.
    Left,
    /// `50% 50%` to the viewport.
    Center,
    /// `100% 50%` to the viewport.
    Right,
    /// `0% 100%` to the viewport.
    LeftBottom,
    /// `50% 100%` to the viewport.
    Bottom,
    /// `100% 100%` to the viewport.
    RightBottom,
    Standard(String),
    Arbitrary,
}

impl AnchorPoint {
    pub fn parse(
        pattern: &[&str],
        arbitrary: &TailwindArbitrary,
        allow_center: bool,
    ) -> Result<Self> {
        let out = match pattern {
            ["7" | "tl" | "lt"] | ["left", "top"] | ["top", "left"] => Self::LeftTop,
            ["8" | "t"] | ["top"] => Self::Top,
            ["9" | "rt" | "tr"] | ["right", "top"] | ["top", "right"] => Self::RightTop,
            ["4" | "l"] | ["left"] => Self::Left,
            ["5" | "c"] | ["center"] if allow_center => Self::Center,
            ["6" | "r"] | ["right"] => Self::Right,
            ["1" | "lb" | "bl"] | ["left", "bottom"] | ["bottom", "left"] => Self::LeftBottom,
            ["2" | "b"] | ["bottom"] => Self::Bottom,
            ["3" | "rb" | "br"] | ["right", "bottom"] | ["bottom", "right"] => Self::RightBottom,
            [n] if Self::check_valid(n) => Self::Standard(n.to_string()),
            [] => Self::Arbitrary,
            _ => return syntax_error!("Unknown anchor-point instructions: {}", pattern.join("-")),
        };
        Ok(out)
    }

    pub fn check_valid(mode: &str) -> bool {
        ["inherit", "initial", "revert", "unset"].contains(&mode)
    }
}
