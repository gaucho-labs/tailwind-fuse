use super::*;

#[derive(Clone, Debug)]
pub struct TailwindInvert {
    percent: NumericValue,
    backdrop: Backdrop,
}

impl TailwindInstance for TailwindInvert {
    fn collision_id(&self) -> &'static str {
        if self.backdrop.0 {
            "backdrop-invert"
        } else {
            "invert"
        }
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
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
