use super::*;

#[derive(Clone, Debug)]
pub struct TailwindShadowColor {
    color: TailwindColor,
}

crate::macros::color_instance!(TailwindShadowColor => "shadow-color");
