use super::*;

pub(crate) mod flex_direction;
pub(crate) mod flex_wrap;

#[derive(Debug, Clone)]
pub struct TailwindFlex {
    kind: NumericValue,
}

impl Display for TailwindFlex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "flex-{}", self.kind)
    }
}

// TODO: CONFIRM, this is prolly ok?
impl TailwindInstance for TailwindFlex {
    fn collision_id(&self) -> String {
        "base-flex".into()
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindFlex {
    pub fn adapt(
        pattern: &[&str],
        arbitrary: &TailwindArbitrary,
    ) -> Result<Box<dyn TailwindInstance>> {
        let out = match pattern {
            // https://tailwindcss.com/docs/display#flex
            // This won't happen
            [] if arbitrary.is_none() => TailwindDisplay::from("flex").boxed(),
            // https://tailwindcss.com/docs/flex#arbitrary-values
            [] => TailwindFlex::parse_arbitrary(arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/flex-direction
            ["row"] => TailwindFlexDirection::from("row").boxed(),
            ["row", "reverse"] => TailwindFlexDirection::from("row-reverse").boxed(),
            ["col"] => TailwindFlexDirection::from("column").boxed(),
            ["col", "reverse"] => TailwindFlexDirection::from("column-reverse").boxed(),
            ["direction", rest @ ..] => TailwindFlexDirection::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/flex-wrap
            ["wrap", rest @ ..] => TailwindFlexWrap::parse(rest, arbitrary)?.boxed(),
            ["nowrap"] => TailwindFlexWrap::from("nowrap").boxed(),
            // https://tailwindcss.com/docs/flex
            _ => Self::parse(pattern, arbitrary)?.boxed(),
        };
        Ok(out)
    }
    /// <https://tailwindcss.com/docs/flex>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<TailwindFlex> {
        let kind = NumericValue::positive_parser("flex", &Self::check_valid)(pattern, arbitrary)?;
        Ok(TailwindFlex { kind })
    }
    /// dispatch to [flex](https://developer.mozilla.org/en-US/docs/Web/CSS/flex)
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        NumericValue::parse_arbitrary(arbitrary).map(|kind| Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "auto", "inherit", "initial", "initial", "none", "revert", "unset",
        ]);
        set.contains(mode)
    }
}
