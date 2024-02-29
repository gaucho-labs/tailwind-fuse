use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindTextColor {
    color: TailwindColor,
}

crate::macros::sealed::color_instance!(TailwindTextColor);

impl Display for TailwindTextColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "text-{}", self.color)
    }
}

impl TailwindInstance for TailwindTextColor {
}
