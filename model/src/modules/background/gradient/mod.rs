use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindFrom {
    color: TailwindColor,
}

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindVia {
    color: TailwindColor,
}

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindTo {
    color: TailwindColor,
}
crate::macros::sealed::color_instance!(TailwindFrom);
crate::macros::sealed::color_instance!(TailwindVia);
crate::macros::sealed::color_instance!(TailwindTo);

impl Display for TailwindFrom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "from-{}", self.color)
    }
}
impl Display for TailwindVia {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "via-{}", self.color)
    }
}
impl Display for TailwindTo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "to-{}", self.color)
    }
}
impl TailwindInstance for TailwindFrom {}
impl TailwindInstance for TailwindVia {}
impl TailwindInstance for TailwindTo {}
