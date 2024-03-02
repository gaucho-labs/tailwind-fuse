use super::*;

#[derive(Clone, Debug)]
pub struct TailwindOutlineColor {
    color: TailwindColor,
}

crate::macros::color_instance!(TailwindOutlineColor => "outline-color");
