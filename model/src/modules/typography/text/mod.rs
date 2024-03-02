use super::*;

pub(crate) mod text_align;
pub(crate) mod text_color;
pub(crate) mod text_overflow;
pub(crate) mod text_transform;

pub fn text_adaptor(
    pattern: &[&str],
    arbitrary: &TailwindArbitrary,
) -> Result<Box<dyn TailwindInstance>> {
    let out = match pattern {
        // https://tailwindcss.com/docs/text-align
        [s @ ("left" | "center" | "right" | "justify" | "start" | "end")] => {
            TailwindTextAlignment::try_from(*s)?.boxed()
        }
        // https://tailwindcss.com/docs/text-overflow
        [s @ ("ellipsis" | "clip")] => TailwindTextOverflow::try_from(*s)?.boxed(),
        // https://tailwindcss.com/docs/text-transform
        [s @ ("uppercase" | "lowercase" | "capitalize" | "normal-case")] => {
            TailwindTextTransform::try_from(*s)?.boxed()
        }
        // https://tailwindcss.com/docs/font-size
        [s @ ("xs" | "sm" | "md" | "lg" | "xl" | "2xl" | "3xl" | "4xl" | "5xl" | "6xl" | "7xl"
        | "8xl" | "9xl")] => TailwindFontSize::new(s).boxed(),
        // https://tailwindcss.com/docs/text-color
        _ => {
            // TODO: Font needs to be compared to color here!
            let color = TailwindColor::parse(pattern, arbitrary)?;
            TailwindTextColor::from(color).boxed()
        }
    };
    Ok(out)
}
