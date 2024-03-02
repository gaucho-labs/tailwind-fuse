use super::*;

pub(crate) mod grid_auto;
pub(crate) mod grid_cols;
pub(crate) mod grid_flow;
pub(crate) mod grid_rows;

#[derive(Debug, Clone, Copy)]
pub struct TailwindGrid {}

impl TailwindGrid {
    pub fn adapt(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/grid-template-rows
            ["rows", rest @ ..] => TailwindGridRows::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/grid-template-columns
            ["cols", rest @ ..] => TailwindGridColumns::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/grid-auto-flow
            ["flow", rest @ ..] => TailwindGridFlow::parse(rest, arbitrary)?.boxed(),
            _ => return syntax_error!("Unknown list instructions: {}", str.join("-")),
        };
        Ok(out)
    }
}

#[derive(Debug, Clone)]
enum GridTemplate {
    None,
    Unit(i32),
    Arbitrary,
}

impl GridTemplate {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            ["none"] => Self::None,
            [n] => Self::Unit(TailwindArbitrary::from(*n).as_integer()?),
            _ => Self::Arbitrary,
        };
        Ok(kind)
    }
}
