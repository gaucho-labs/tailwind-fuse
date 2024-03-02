use super::*;

#[derive(Clone, Debug)]
pub struct TailwindScrollPadding {
    negative: Negative,
    axis: SpacingAxis,
    size: SpacingSize,
}

crate::spacing_collision!(TailwindScrollPadding => "scroll-padding");

impl TailwindScrollPadding {
    /// https://tailwindcss.com/docs/scroll-padding
    pub fn parse(
        pattern: &[&str],
        arbitrary: &TailwindArbitrary,
        negative: Negative,
    ) -> Result<Self> {
        let (axis, rest) = match pattern {
            ["p", rest @ ..] => (SpacingAxis::All, rest),
            ["pl", rest @ ..] => (SpacingAxis::Left, rest),
            ["pr", rest @ ..] => (SpacingAxis::Right, rest),
            ["pt", rest @ ..] => (SpacingAxis::Top, rest),
            ["pb", rest @ ..] => (SpacingAxis::Bottom, rest),
            ["px", rest @ ..] => (SpacingAxis::X, rest),
            ["py", rest @ ..] => (SpacingAxis::Y, rest),
            _ => return syntax_error!("Unknown scroll-padding axis"),
        };
        let size = SpacingSize::parse(rest, arbitrary, &Self::check_valid)?;
        Ok(Self {
            negative,
            axis,
            size,
        })
    }
    /// https://tailwindcss.com/docs/scroll-padding#arbitrary-values
    pub fn parse_arbitrary(
        arbitrary: &TailwindArbitrary,
        axis: SpacingAxis,
        negative: Negative,
    ) -> Result<Self> {
        let size = SpacingSize::parse_arbitrary(arbitrary)?;
        Ok(Self {
            negative,
            axis,
            size,
        })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/padding#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
}
