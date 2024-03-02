use super::*;

#[derive(Clone, Debug)]
pub struct TailwindCaretColor {
    color: TailwindColor,
}

crate::macros::color_instance!(TailwindCaretColor => "caret-color");
