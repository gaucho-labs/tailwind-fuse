use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindDivideColor {
    color: TailwindColor,
}

crate::macros::sealed::color_instance!(TailwindDivideColor);

impl Display for TailwindDivideColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ring-{}", self.color)
    }
}

impl TailwindInstance for TailwindDivideColor {}
