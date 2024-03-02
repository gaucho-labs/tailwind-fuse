use super::*;

#[derive(Clone, Debug)]
pub struct TailwindDivideReverse {
    axis: bool,
}

impl From<bool> for TailwindDivideReverse {
    fn from(axis: bool) -> Self {
        Self { axis }
    }
}

impl TailwindInstance for TailwindDivideReverse {
    // TODO: CONFIRM
    fn collision_id(&self) -> &'static str {
        if self.axis {
            "divide-x-reverse"
        } else {
            "divide-y-reverse"
        }
    }

    // TODO: confirm?
    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}
