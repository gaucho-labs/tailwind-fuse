use std::fmt::{Display, Formatter};

use crate::TailwindError;

pub use self::{
    anchor::AnchorPoint, axis::SpacingAxis, axis_xy::AxisXY, integer_only::NumericValue,
    keyword_only::StandardValue, length::LengthUnit, length_only::UnitValue, negative::Negative,
};
use crate::{syntax_error, Result, TailwindArbitrary};

mod anchor;
mod axis;
mod axis_xy;
mod integer_only;
mod keyword_only;
mod length;
mod length_only;
mod negative;
