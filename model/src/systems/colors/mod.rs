mod color;
mod palette;
mod palette_system;

pub use self::{color::TailwindColor, palette::Palette, palette_system::PaletteSystem};
use crate::{syntax_error, Result, TailwindArbitrary};
use css_color::Srgb;
use std::{
    collections::{BTreeMap, HashMap},
    fmt::{Display, Formatter},
    str::FromStr,
};
