mod animate;
mod delay;
mod duration;
mod ease;
mod transit;
pub use self::{
    animate::TailwindAnimate, delay::TailwindDelay, duration::TailwindDuration, ease::TailwindEase,
    transit::TailwindTransition,
};
use crate::{syntax_error, NumericValue, Result, TailwindArbitrary, TailwindInstance};
use std::collections::BTreeSet;
