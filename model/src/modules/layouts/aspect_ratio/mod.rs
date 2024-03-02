use super::*;
mod aspect;

use self::aspect::Aspect;

#[derive(Clone, Debug)]
pub struct TailwindAspect {
    kind: Aspect,
}

impl<T> From<T> for TailwindAspect
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self {
            kind: Aspect::Standard(kind.into()),
        }
    }
}

impl TailwindInstance for TailwindAspect {
    fn collision_id(&self) -> &'static str {
        "aspect-ratio"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindAspect {
    /// <https://tailwindcss.com/docs/aspect-ratio>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            kind: Aspect::parse(pattern, arbitrary)?,
        })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/aspect-ratio>
    pub fn check_valid(mode: &str) -> bool {
        Aspect::check_valid(mode)
    }
}
