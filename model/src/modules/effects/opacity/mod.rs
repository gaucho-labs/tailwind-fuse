use super::*;

#[derive(Clone, Debug)]
pub struct TailwindOpacity {
    percent: NumericValue,
    backdrop: Backdrop,
}

impl Display for TailwindOpacity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.backdrop.write(f)?;
        write!(f, "opacity-{}", self.percent)
    }
}

impl TailwindInstance for TailwindOpacity {
    fn collision_id(&self) -> String {
        if self.backdrop.0 {
            "backdrop-opacity".into()
        } else {
            "opacity".into()
        }
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindOpacity {
    /// <https://tailwindcss.com/docs/opacity>
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        let backdrop = Backdrop::from(backdrop);
        if input.is_empty() {
            return Ok(Self {
                percent: NumericValue::from(50u32),
                backdrop,
            });
        };
        let percent = match backdrop.0 {
            true => NumericValue::positive_parser("opacity", |_| false)(input, arbitrary)?,
            false => NumericValue::positive_parser("opacity", Self::check_valid)(input, arbitrary)?,
        };
        Ok(Self { percent, backdrop })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/opacity#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["always", "inherit", "initial", "normal", "unset"]);
        set.contains(mode)
    }
}
