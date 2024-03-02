use super::*;

#[derive(Clone, Debug)]
pub struct TailwindRingColor {
    color: TailwindColor,
}

crate::macros::color_instance!(TailwindRingColor => "ring-color");
