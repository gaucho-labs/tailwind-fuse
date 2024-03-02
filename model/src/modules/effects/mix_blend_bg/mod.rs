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
    pub fn parse(input: &[&str]) -> Result<Self> {
        let wrapper = TailwindBlend::try_from(input)?;
        Ok(Self { wrapper })
    }
}
