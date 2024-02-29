use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBackgroundColor {
    color: TailwindColor,
}
crate::macros::sealed::color_instance!(TailwindBackgroundColor);

impl Display for TailwindBackgroundColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "bg-{}", self.color)
    }
}

impl TailwindInstance for TailwindBackgroundColor {}
