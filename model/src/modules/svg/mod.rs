pub use self::{
    fill::TailwindFillColor,
    stroke::{
        stroke_color::TailwindStrokeColor, stroke_width::TailwindStrokeWidth, TailwindStroke,
    },
};
use crate::{LengthUnit, Result, TailwindArbitrary, TailwindColor, TailwindInstance};

mod fill;
mod stroke;
