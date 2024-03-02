use super::*;

#[derive(Clone, Debug)]
pub struct TailwindBlur {
    px: NumericValue,
    backdrop: Backdrop,
}

impl TailwindInstance for TailwindBlur {
    fn collision_id(&self) -> &'static str {
        if self.backdrop.0 {
            "backdrop-blur"
        } else {
            "blur"
        }
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindBlur {
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        let px = match rest {
            [] if arbitrary.is_none() => 8u32.into(),
            _ => NumericValue::positive_parser("blur", |_| false)(rest, arbitrary)?,
        };
        Ok(Self {
            px,
            backdrop: Backdrop::from(backdrop),
        })
    }
}
