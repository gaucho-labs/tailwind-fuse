use super::*;

/// <https://tailwindcss.com/docs/overscroll-behavior>
/// https://developer.mozilla.org/en-US/docs/Web/CSS/overscroll-behavior#syntax
#[derive(Clone, Debug)]
pub struct TailwindOverscroll {
    axis: AxisXY,
}

crate::axisxy_collision!(TailwindOverscroll => "overscroll");

impl TailwindOverscroll {
    pub fn parse(pattern: &[&str]) -> Self {
        let (axis, _) = AxisXY::split_xyn(pattern);
        Self { axis }
    }
}
