use super::*;
use crate::Negative;

#[derive(Clone, Debug)]
pub struct TailwindHueRotate {
    degree: NumericValue,
    backdrop: Backdrop,
}

impl Display for TailwindHueRotate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.backdrop.write(f)?;
        write!(f, "hue-rotate-{}", self.degree)
    }
}

impl TailwindInstance for TailwindHueRotate {
    fn collision_id(&self) -> String {
        if self.backdrop.0 {
            "backdrop-hue-rotate".into()
        } else {
            "hue-rotate".into()
        }
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindHueRotate {
    /// <https://tailwindcss.com/docs/hue-rotate>
    pub fn parse(
        rest: &[&str],
        arbitrary: &TailwindArbitrary,
        backdrop: bool,
        negative: Negative,
    ) -> Result<Self> {
        let degree = match rest {
            [] if arbitrary.is_none() => 180u32.into(),
            _ => NumericValue::negative_parser("hue-rotate", |_| false)(rest, arbitrary, negative)?,
        };
        Ok(Self {
            degree,
            backdrop: Backdrop::from(backdrop),
        })
    }
}
