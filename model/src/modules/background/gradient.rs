use super::*;

#[derive(Clone, Debug)]
pub struct TailwindFrom {
    color: TailwindColor,
}

#[derive(Clone, Debug)]
pub struct TailwindVia {
    color: TailwindColor,
}

#[derive(Clone, Debug)]
pub struct TailwindTo {
    color: TailwindColor,
}
crate::macros::color_instance!(TailwindFrom => "from");
crate::macros::color_instance!(TailwindVia => "via");
crate::macros::color_instance!(TailwindTo => "to");
