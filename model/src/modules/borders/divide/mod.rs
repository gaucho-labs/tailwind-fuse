pub(crate) mod divide_color;
pub(crate) mod divide_reverse;
pub(crate) mod divide_style;
pub(crate) mod divide_width;

use super::*;

#[derive(Copy, Clone, Debug, Default)]
pub struct TailwindDivide {}

impl TailwindDivide {
    /// Parse the instructions starting with `divide`.
    pub fn adapt(
        pattern: &[&str],
        arbitrary: &TailwindArbitrary,
    ) -> Result<Box<dyn TailwindInstance>> {
        let out = match pattern {
            // https://tailwindcss.com/docs/divide-width
            ["x", "reverse"] => TailwindDivideReverse::from(true).boxed(),
            ["y", "reverse"] => TailwindDivideReverse::from(false).boxed(),
            ["x", rest @ ..] => TailwindDivideWidth::parse(rest, arbitrary, true)?.boxed(),
            ["y", rest @ ..] => TailwindDivideWidth::parse(rest, arbitrary, false)?.boxed(),
            // https://tailwindcss.com/docs/divide-style
            ["solid" | "dashed" | "dotted" | "double" | "none"] => {
                TailwindDivideStyle::parse(pattern)?.boxed()
            }
            ["style", rest @ ..] => TailwindDivideStyle::parse(rest)?.boxed(),
            // https://tailwindcss.com/docs/divide-color
            _ => return syntax_error!("Unknown divide instructions: {}", pattern.join("-")),
        };
        Ok(out)
    }
}
