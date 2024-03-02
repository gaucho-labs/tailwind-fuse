use super::*;

#[derive(Clone, Debug)]
pub struct TailwindGrayscale {
    percent: NumericValue,
    backdrop: Backdrop,
}

impl TailwindInstance for TailwindGrayscale {
    fn collision_id(&self) -> &'static str {
        if self.backdrop.0 {
            "backdrop-grayscale"
        } else {
            "grayscale"
        }
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
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
