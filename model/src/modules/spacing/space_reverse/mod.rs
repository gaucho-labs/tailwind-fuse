use crate::Axis2D;

use super::*;

#[derive(Clone, Debug)]
pub struct TailwindSpaceReverse {
    pub(crate) axis: Axis2D,
}

crate::axis2d_collision!(TailwindSpaceReverse => "space");
