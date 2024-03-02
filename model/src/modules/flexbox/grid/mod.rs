use crate::Axis2D;

use super::*;

#[derive(Debug, Clone, Copy)]
pub struct TailwindGrid {}

impl TailwindGrid {
    pub fn adapt(str: &[&str], _: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/grid-template-rows
            ["rows", ..] => TailwindGridRows {}.boxed(),
            // https://tailwindcss.com/docs/grid-template-columns
            ["cols", ..] => TailwindGridColumns {}.boxed(),
            // https://tailwindcss.com/docs/grid-auto-flow
            ["flow", rest @ ..] => TailwindGridFlow::try_from(rest.join("-").as_str())?.boxed(),
            _ => return syntax_error!("Unknown list instructions: {}", str.join("-")),
        };
        Ok(out)
    }
}

#[derive(Debug, Clone)]
pub struct TailwindGridRows {}

impl TailwindInstance for TailwindGridRows {
    fn collision_id(&self) -> &'static str {
        "grid-rows"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

#[derive(Debug, Clone)]
pub struct TailwindGridColumns {}

impl TailwindInstance for TailwindGridColumns {
    fn collision_id(&self) -> &'static str {
        "grid-columns"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

#[derive(Debug, Clone)]
pub struct TailwindGridAuto {
    axis: Axis2D,
}

impl TailwindGridAuto {
    pub fn parse(pattern: &[&str]) -> Result<Self> {
        let axis = match pattern {
            ["rows", ..] => Axis2D::X,
            ["cols", ..] => Axis2D::Y,
            _ => return syntax_error!("Unknown auto instructions: {}", pattern.join("-")),
        };
        Ok(Self { axis })
    }
}

crate::macros::axis2d_collision!(TailwindGridAuto => "grid-auto");

#[derive(Debug, Clone)]
pub struct TailwindGridFlow {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindGridFlow => "grid-auto-flow",[
    "row", "col", "dense", "row-dense", "col-dense"
]);
