use super::*;

#[derive(Debug, Clone)]
pub struct TailwindTextColor {
    color: TailwindColor,
}

crate::macros::color_instance!(TailwindTextColor => "text-color");
