#[derive(Debug, Clone, Copy)]
pub enum Axis2D {
    X,
    Y,
}

impl From<bool> for Axis2D {
    fn from(axis: bool) -> Self {
        if axis {
            Axis2D::X
        } else {
            Axis2D::Y
        }
    }
}
