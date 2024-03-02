use super::*;

#[derive(Clone, Debug)]
pub struct TailwindBorderColor {
    color: TailwindColor,
}

crate::macros::color_instance!(TailwindBorderColor => "border-color");
