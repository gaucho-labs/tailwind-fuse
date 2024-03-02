use super::*;

#[derive(Debug, Clone)]
pub struct TailwindBreak {
    kind: WordBreak,
}

#[derive(Debug, Clone)]
enum WordBreak {
    Normal,
    Words,
    Standard(String),
}

impl<T> From<T> for TailwindBreak
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self {
            kind: WordBreak::Standard(kind.into()),
        }
    }
}

impl TailwindInstance for TailwindBreak {
    fn collision_id(&self) -> &'static str {
        "work-break".into()
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindBreak {
    /// https://tailwindcss.com/docs/word-break
    pub fn parse(
        pattern: &[&str],
        arbitrary: &TailwindArbitrary,
    ) -> Result<Box<dyn TailwindInstance>> {
        let kind = match pattern {
            // https://tailwindcss.com/docs/break-before
            ["before", rest @ ..] => TailwindBreakBefore::try_from(rest)?.boxed(),
            // https://tailwindcss.com/docs/break-inside
            ["inside", rest @ ..] => TailwindBreakInside::try_from(rest)?.boxed(),
            // https://tailwindcss.com/docs/break-after
            ["after", rest @ ..] => TailwindBreakAfter::try_from(rest)?.boxed(),
            // https://tailwindcss.com/docs/word-break
            _ => Self::parse_self(pattern, arbitrary)?.boxed(),
        };
        Ok(kind)
    }
    fn parse_self(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            kind: WordBreak::parse(pattern, arbitrary)?,
        })
    }
}

impl WordBreak {
    pub fn parse(pattern: &[&str], _arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            ["normal"] => Self::Normal,
            ["words"] => Self::Words,
            ["all"] => Self::Standard("break-all".to_string()),
            _ => {
                let kind = pattern.join("-");
                debug_assert!(Self::check_valid(&kind));
                Self::Standard(kind)
            }
        };
        Ok(kind)
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/word-break#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "break-all",
            "inherit",
            "initial",
            "keep-all",
            "normal",
            "revert",
            "unset",
        ]);
        set.contains(mode)
    }
}
