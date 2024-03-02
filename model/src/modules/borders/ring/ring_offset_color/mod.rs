use super::*;

#[derive(Clone, Debug)]
pub struct TailwindRingOffsetColor {
    color: TailwindColor,
}

crate::macros::color_instance!(TailwindRingOffsetColor => "ring-offset-color");
