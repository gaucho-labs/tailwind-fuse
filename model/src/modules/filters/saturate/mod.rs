use super::*;

#[derive(Clone, Debug)]
pub struct TailwindSaturate {
    percent: NumericValue,
    backdrop: Backdrop,
}

impl TailwindInstance for TailwindSaturate {
    fn collision_id(&self) -> &'static str {
        "saturate"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindSaturate {
    /// <https://tailwindcss.com/docs/saturate>
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        let percent = match rest {
            [] if arbitrary.is_none() => 100u32.into(),
            _ => NumericValue::positive_parser("saturate", |_| false)(rest, arbitrary)?,
        };
        Ok(Self {
            percent,
            backdrop: Backdrop::from(backdrop),
        })
    }
}
