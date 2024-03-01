use super::*;
use crate::StandardValue;

#[derive(Clone, Debug)]
pub struct TailwindDivideStyle {
    kind: StandardValue,
}

impl<T> From<T> for TailwindDivideStyle
where
    T: Into<String>,
{
    fn from(input: T) -> Self {
        Self {
            kind: StandardValue::from(input.into()),
        }
    }
}

impl Display for TailwindDivideStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "divide-{}", self.kind)
    }
}

impl TailwindInstance for TailwindDivideStyle {
    fn collision_id(&self) -> String {
        "divide-style".into()
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindDivideStyle {
    /// https://tailwindcss.com/docs/divide-style
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("divide-style", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/border-style#syntax
    pub fn check_valid(mode: &str) -> bool {
        TailwindBorderStyle::check_valid(mode)
    }
}
