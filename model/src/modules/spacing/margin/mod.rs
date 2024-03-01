use super::*;

#[derive(Clone, Debug)]
pub struct TailwindMargin {
    negative: Negative,
    axis: SpacingAxis,
    size: SpacingSize,
}

// noinspection DuplicatedCode
impl Display for TailwindMargin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.negative.write(f)?;
        write!(f, "{}-{}", self.axis, self.size)
    }
}

// noinspection DuplicatedCode
impl TailwindInstance for TailwindMargin {
    fn collision_id(&self) -> String {
        self.axis.collision_id("margin")
    }

    fn get_collisions(&self) -> Vec<String> {
        self.axis.collisions("margin")
    }
}

// noinspection DuplicatedCode
impl TailwindMargin {
    /// https://tailwindcss.com/docs/margin
    pub fn parse(
        pattern: &[&str],
        arbitrary: &TailwindArbitrary,
        negative: Negative,
    ) -> Result<Self> {
        let (axis, rest) = match pattern {
            ["m", rest @ ..] => (SpacingAxis::All, rest),
            ["ml", rest @ ..] => (SpacingAxis::Left, rest),
            ["mr", rest @ ..] => (SpacingAxis::Right, rest),
            ["mt", rest @ ..] => (SpacingAxis::Top, rest),
            ["mb", rest @ ..] => (SpacingAxis::Bottom, rest),
            ["mx", rest @ ..] => (SpacingAxis::X, rest),
            ["my", rest @ ..] => (SpacingAxis::Y, rest),
            _ => return syntax_error!("Unknown margin axis"),
        };
        let size = SpacingSize::parse(rest, arbitrary, &Self::check_valid)?;
        Ok(Self {
            negative,
            axis,
            size,
        })
    }
    /// https://tailwindcss.com/docs/margin#arbitrary-values
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
        let set = BTreeSet::from_iter(vec!["auto", "inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
}
