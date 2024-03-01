use super::*;

#[derive(Clone, Debug)]
pub struct TailwindPadding {
    negative: Negative,
    axis: SpacingAxis,
    size: SpacingSize,
}

// noinspection DuplicatedCode
impl Display for TailwindPadding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.negative.write(f)?;
        write!(f, "{}-{}", self.axis, self.size)
    }
}

// noinspection DuplicatedCode
impl TailwindInstance for TailwindPadding {
    fn collision_id(&self) -> String {
        self.axis.collision_id("padding")
    }

    fn get_collisions(&self) -> Vec<String> {
        self.axis.collisions("padding")
    }
}

// noinspection DuplicatedCode
impl TailwindPadding {
    /// https://tailwindcss.com/docs/padding
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
            _ => return syntax_error!("Unknown padding axis"),
        };
        let size = SpacingSize::parse(rest, arbitrary, &Self::check_valid)?;
        Ok(Self {
            negative,
            axis,
            size,
        })
    }
    /// https://tailwindcss.com/docs/padding#arbitrary-values
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
