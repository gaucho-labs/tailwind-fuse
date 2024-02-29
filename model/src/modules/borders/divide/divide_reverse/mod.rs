use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindDivideReverse {
    axis: bool,
}

impl From<bool> for TailwindDivideReverse {
    fn from(axis: bool) -> Self {
        Self { axis }
    }
}

impl Display for TailwindDivideReverse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.axis {
            true => write!(f, "divide-x-reverse"),
            false => write!(f, "divide-y-reverse"),
        }
    }
}

impl TailwindInstance for TailwindDivideReverse {
    // TODO: CONFIRM
    fn collision_id(&self) -> String {
        self.to_string()
    }

    fn get_collisions(&self) -> Vec<String> {
        vec![self.collision_id()]
    }
}
