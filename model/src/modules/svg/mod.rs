pub use self::{
    fill::TailwindFillColor,
    stroke::{stroke_color::TailwindStrokeColor, stroke_width::TailwindStrokeWidth, TailwindStroke},
};
use crate::{LengthUnit, Result, TailwindArbitrary, TailwindColor, TailwindInstance};
use std::fmt::{Display, Formatter};

mod fill;
mod stroke;
