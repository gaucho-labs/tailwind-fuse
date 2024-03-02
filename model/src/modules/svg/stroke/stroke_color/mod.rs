use super::*;

#[derive(Debug, Clone)]
pub struct TailwindStrokeColor {
    color: TailwindColor,
}

crate::macros::color_instance!(TailwindStrokeColor => "stroke-color");
