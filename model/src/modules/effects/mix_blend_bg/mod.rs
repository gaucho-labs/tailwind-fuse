use super::*;

#[derive(Clone, Debug)]
pub struct TailwindBackgroundBlend {
    wrapper: TailwindBlend,
}

impl Display for TailwindBackgroundBlend {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "bg-blend-{}", self.wrapper.get_class())
    }
}

impl TailwindInstance for TailwindBackgroundBlend {}

impl TailwindBackgroundBlend {
    /// <https://tailwindcss.com/docs/background-blend-mode>
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { wrapper: TailwindBlend::parse(input, arbitrary)? })
    }
    pub fn check_valid(input: &str) -> bool {
        TailwindBlend::check_valid(input)
    }
}
