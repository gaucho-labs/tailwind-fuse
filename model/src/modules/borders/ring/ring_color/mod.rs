use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindRingColor {
    color: TailwindColor,
}

crate::macros::sealed::color_instance!(TailwindRingColor);

impl Display for TailwindRingColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ring-{}", self.color)
    }
}

impl TailwindInstance for TailwindRingColor {}
