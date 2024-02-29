use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindShadowColor {
    color: TailwindColor,
}

crate::macros::sealed::color_instance!(TailwindShadowColor);

impl Display for TailwindShadowColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "shadow-{}", self.color)
    }
}

impl TailwindInstance for TailwindShadowColor {}
