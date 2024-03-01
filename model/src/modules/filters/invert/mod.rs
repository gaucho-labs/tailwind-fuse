use super::*;

#[derive(Clone, Debug)]
pub struct TailwindInvert {
    percent: NumericValue,
    backdrop: Backdrop,
}
impl Display for TailwindInvert {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.backdrop.write(f)?;
        write!(f, "invert-{}", self.percent)
    }
}

impl TailwindInstance for TailwindInvert {
    fn collision_id(&self) -> String {
        if self.backdrop.0 {
            "backdrop-invert".to_string()
        } else {
            "invert".to_string()
        }
    }

    fn get_collisions(&self) -> Vec<String> {
        vec![self.collision_id()]
    }
}

impl TailwindInvert {
    /// <https://tailwindcss.com/docs/invert>
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        let percent = match rest {
            [] if arbitrary.is_none() => 100u32.into(),
            _ => NumericValue::positive_parser("invert", |_| false)(rest, arbitrary)?,
        };
        Ok(Self {
            percent,
            backdrop: Backdrop::from(backdrop),
        })
    }
}
