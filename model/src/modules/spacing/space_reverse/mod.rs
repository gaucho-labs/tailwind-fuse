use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindSpaceReverse {
    axis: bool,
}

impl From<bool> for TailwindSpaceReverse {
    fn from(axis: bool) -> Self {
        Self { axis }
    }
}

impl Display for TailwindSpaceReverse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.axis {
            true => write!(f, "space-x-reverse"),
            false => write!(f, "space-y-reverse"),
        }
    }
}

impl TailwindInstance for TailwindSpaceReverse {
}
