use super::*;

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
    fn collision_id(&self) -> String {
        if self.axis {
            "space-x-reverse".to_string()
        } else {
            "space-y-reverse".to_string()
        }
    }

    fn get_collisions(&self) -> Vec<String> {
        vec![self.collision_id()]
    }
}
