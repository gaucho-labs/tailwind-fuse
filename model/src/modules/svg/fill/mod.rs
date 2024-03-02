use super::*;

#[derive(Debug, Clone)]
pub struct TailwindFillColor {
    color: TailwindColor,
}

crate::macros::color_instance!(TailwindFillColor=> "fill-color");
