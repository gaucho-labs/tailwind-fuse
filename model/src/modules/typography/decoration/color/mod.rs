use super::*;

#[derive(Debug, Clone)]
pub struct TailwindDecorationColor {
    color: TailwindColor,
}

crate::macros::color_instance!(TailwindDecorationColor => "decoration-color");
