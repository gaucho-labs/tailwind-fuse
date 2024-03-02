use super::*;

#[derive(Clone, Debug)]
pub struct TailwindBackgroundBlend {
    wrapper: TailwindBlend,
}

impl TailwindInstance for TailwindBackgroundBlend {
    fn collision_id(&self) -> &'static str {
        "background-blend"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindBackgroundBlend {
    /// <https://tailwindcss.com/docs/background-blend-mode>
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            wrapper: TailwindBlend::parse(input, arbitrary)?,
        })
    }
    pub fn check_valid(input: &str) -> bool {
        TailwindBlend::check_valid(input)
    }
}
