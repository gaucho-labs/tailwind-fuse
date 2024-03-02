pub use self::{
    box_shadow::TailwindShadow, mix_blend::TailwindBlend, mix_blend_bg::TailwindBackgroundBlend,
    opacity::TailwindOpacity, shadow_color::TailwindShadowColor,
};
use crate::{
    Backdrop, NumericValue, Result, StandardValue, TailwindArbitrary, TailwindColor,
    TailwindInstance,
};
use std::collections::BTreeSet;
mod box_shadow;
mod mix_blend;
mod mix_blend_bg;
mod opacity;
mod shadow_color;
