use super::*;

pub(crate) mod color;
pub(crate) mod line;
pub(crate) mod style;
pub(crate) mod thickness;

#[derive(Debug, Clone)]
pub struct TailwindDecoration {}

// TODO: COLOR AND THICKNESS ARE BOTH CUSTOMIZABLE

impl TailwindDecoration {
    pub fn adapt(
        pattern: &[&str],
        arbitrary: &TailwindArbitrary,
    ) -> Result<Box<dyn TailwindInstance>> {
        let out = match pattern {
            // https://tailwindcss.com/docs/text-decoration
            ["line", rest @ ..] => TailwindDecorationLine::try_from(rest)?.boxed(),
            // https://tailwindcss.com/docs/text-decoration-style
            [s @ ("solid" | "double" | "dotted" | "dashed" | "wavy")] => {
                TailwindDecorationStyle::try_from(*s)?.boxed()
            }
            // https://tailwindcss.com/docs/text-decoration-thickness
            ["auto"] => TailwindDecorationThickness::from("auto").boxed(),
            ["from", "font"] => TailwindDecorationThickness::from("from-font").boxed(),

            // At this point we need to decide between decoration color and thickness
            // If it's a number or arbitrary len, it's thickness
            // Otherwise, it's color
            // https://tailwindcss.com/docs/text-decoration-color
            [n] => resolve1(n)?,
            [] => resolve1(arbitrary.as_str())?,
            _ => syntax_error!("Invalid decoration pattern")?,
        };
        Ok(out)
    }
}

impl TailwindInstance for TailwindDecoration {
    fn collision_id(&self) -> &'static str {
        "decoration"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

fn resolve1(n: &str) -> Result<Box<dyn TailwindInstance>> {
    let a = TailwindArbitrary::from(n);
    if n.starts_with(|c: char| c.is_numeric()) {
        return Ok(resolve1_unit(&a)?.boxed());
    }
    if n.starts_with(|c: char| c == '#') {
        // TODO: Can you use non-hex colors?
        return Ok(TailwindDecorationColor::from(TailwindColor::Arbitrary).boxed());
    }
    Ok(TailwindDecorationColor::from(TailwindColor::Themed(n.to_string(), 0)).boxed())
}

fn resolve1_unit(a: &TailwindArbitrary) -> Result<TailwindDecorationThickness> {
    Ok(TailwindDecorationThickness::from(a.as_integer()?))
}
