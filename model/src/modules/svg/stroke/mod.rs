use crate::syntax_error;

use super::*;

pub(crate) mod stroke_color;
pub(crate) mod stroke_width;

#[derive(Clone, Debug)]
pub struct TailwindStroke {}

impl TailwindStroke {
    pub fn parse(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/text-decoration-color
            ["black"] => from_color(TailwindColor::Black),
            ["white"] => from_color(TailwindColor::White),
            // TODO: THIS LOOKS WEIRD?
            ["color"] => from_color(TailwindColor::Arbitrary),
            ["color", rest] => from_color(TailwindColor::Arbitrary),
            // https://tailwindcss.com/docs/text-decoration-color
            [theme, weight] => from_color(TailwindColor::parse_themed(theme, weight)?),
            // https://tailwindcss.com/docs/text-decoration-thickness
            [n] => TailwindStrokeWidth::from_width(n)?.boxed(),
            [] => TailwindStrokeWidth::from_arbitrary(arbitrary)?.boxed(),
            _ => return syntax_error!("Unknown decoration instructions: {}", str.join("-")),
        };
        Ok(out)
    }
}

fn from_color(color: TailwindColor) -> Box<dyn TailwindInstance> {
    TailwindStrokeColor::from(color).boxed()
}
