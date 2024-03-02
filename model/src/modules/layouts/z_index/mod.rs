use super::*;

#[derive(Clone, Debug)]
enum ZIndex {
    Unit(i32),
    Standard(String),
    Arbitrary(TailwindArbitrary),
}

#[derive(Clone, Debug)]
pub struct TailwindZIndex {
    kind: ZIndex,
}

impl TailwindInstance for TailwindZIndex {
    fn collision_id(&self) -> &'static str {
        "z-index"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindZIndex {
    /// <https://tailwindcss.com/docs/z-index>
    pub fn parse(
        pattern: &[&str],
        arbitrary: &TailwindArbitrary,
        negative: Negative,
    ) -> Result<Self> {
        match pattern {
            [] => Self::parse_arbitrary(arbitrary),
            [s] if Self::check_valid(s) => Ok(Self {
                kind: ZIndex::Standard(s.to_string()),
            }),
            [n] => {
                let mut i: i32 = TailwindArbitrary::from(*n).as_integer()?;
                if negative.0 {
                    i = -i;
                }
                Ok(Self {
                    kind: ZIndex::Unit(i),
                })
            }
            _ => syntax_error!("Unknown z-index instructions"),
        }
    }
    /// dispatch to [z-index](https://developer.mozilla.org/en-US/docs/Web/CSS/z-index)
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            kind: ZIndex::Arbitrary(arbitrary.to_owned()),
        })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/z-index#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["auto", "inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
}
