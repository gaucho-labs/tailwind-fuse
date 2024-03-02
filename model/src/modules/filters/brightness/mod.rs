use super::*;

#[derive(Clone, Debug)]
pub struct TailwindBrightness {
    percent: NumericValue,
    backdrop: Backdrop,
}

impl TailwindInstance for TailwindBrightness {
    fn collision_id(&self) -> &'static str {
        if self.backdrop.0 {
            "backdrop-brightness"
        } else {
            "brightness"
        }
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindBrightness {
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        let percent = match rest {
            [] if arbitrary.is_none() => 100u32.into(),
            _ => NumericValue::positive_parser("brightness", |_| false)(rest, arbitrary)?,
        };
        Ok(Self {
            percent,
            backdrop: Backdrop::from(backdrop),
        })
    }
}
