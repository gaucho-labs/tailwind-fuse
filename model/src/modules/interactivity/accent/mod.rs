use super::*;

#[derive(Clone, Debug)]
pub struct TailwindAccentColor {
    color: TailwindColor,
}

crate::macros::color_instance!(TailwindAccentColor => "accent-color");
