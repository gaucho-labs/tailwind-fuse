use super::*;

#[derive(Clone, Debug)]
pub struct TailwindDivideStyle {
    kind: TailwindBorderStyle,
}

impl TailwindInstance for TailwindDivideStyle {
    fn collision_id(&self) -> &'static str {
        "divide-style"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindDivideStyle {
    /// https://tailwindcss.com/docs/divide-style
    pub fn parse(pattern: &[&str]) -> Result<Self> {
        let kind = TailwindBorderStyle::try_from(pattern)?;
        Ok(Self { kind })
    }
}
