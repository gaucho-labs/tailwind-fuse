use std::fmt::{Display, Formatter};

use crate::TailwindError;

pub use self::{
    anchor::AnchorPoint, axis_2d::Axis2D, axis_xy::AxisXY, integer_only::NumericValue,
    keyword_only::StandardValue, length::LengthUnit, length_only::UnitValue, negative::Negative,
    spacing_axis::SpacingAxis,
};
use crate::{syntax_error, Result, TailwindArbitrary};

mod anchor;
mod axis_2d;
mod axis_xy;
mod integer_only;
mod keyword_only;
mod length;
mod length_only;
mod negative;
mod spacing_axis;
