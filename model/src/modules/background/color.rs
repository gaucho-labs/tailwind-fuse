use super::*;

#[derive(Clone, Debug)]
pub struct TailwindBackgroundColor {
    color: TailwindColor,
}

crate::macros::color_instance!(TailwindBackgroundColor => "background-color");
