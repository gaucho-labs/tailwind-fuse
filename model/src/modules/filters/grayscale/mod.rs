use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindGrayscale {
    percent: NumericValue,
    backdrop: Backdrop,
}
impl Display for TailwindGrayscale {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.backdrop.write(f)?;
        write!(f, "grayscale-{}", self.percent)
    }
}

impl TailwindInstance for TailwindGrayscale {
    fn collision_id(&self) -> String {
        if self.backdrop.0 {
            "backdrop-grayscale".into()
        } else {
            "grayscale".into()
        }
    }

    fn get_collisions(&self) -> Vec<String> {
        vec![self.collision_id()]
    }
}

impl TailwindGrayscale {
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        let percent = match rest {
            [] if arbitrary.is_none() => 100u32.into(),
            _ => NumericValue::positive_parser("grayscale", |_| false)(rest, arbitrary)?,
        };
        Ok(Self {
            percent,
            backdrop: Backdrop::from(backdrop),
        })
    }
}
