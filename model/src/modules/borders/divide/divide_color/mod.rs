use super::*;

#[derive(Clone, Debug)]
pub struct TailwindDivideColor {
    color: TailwindColor,
}

crate::macros::color_instance!(TailwindDivideColor => "divide-color");
