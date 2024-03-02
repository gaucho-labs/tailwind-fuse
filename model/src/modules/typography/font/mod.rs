use super::*;

pub(crate) mod font_family;
pub(crate) mod font_size;
pub(crate) mod font_smoothing;
pub(crate) mod font_style;
pub(crate) mod font_variant_numeric;
pub(crate) mod font_weight;

pub fn font_adaptor(
    pattern: &[&str],
    arbitrary: &TailwindArbitrary,
) -> Result<Box<dyn TailwindInstance>> {
    let out = match pattern {
        // https://tailwindcss.com/docs/font-size
        [s @ ("xs" | "sm" | "md" | "lg" | "xl" | "2xl" | "3xl" | "4xl" | "5xl" | "6xl" | "7xl"
        | "8xl" | "9xl")] => TailwindFontSize::new(s).boxed(),
        // https://tailwindcss.com/docs/font-weight
        ["thin"] => TailwindFontWeight::THIN.boxed(),
        ["extralight"] => TailwindFontWeight::EXTRA_LIGHT.boxed(),
        ["light"] => TailwindFontWeight::LIGHT.boxed(),
        ["normal"] => TailwindFontWeight::NORMAL.boxed(),
        ["medium"] => TailwindFontWeight::MEDIUM.boxed(),
        ["semibold"] => TailwindFontWeight::SEMI_BOLD.boxed(),
        ["bold"] => TailwindFontWeight::BOLD.boxed(),
        ["extrabold"] => TailwindFontWeight::EXTRA_BOLD.boxed(),
        ["black"] => TailwindFontWeight::BLACK.boxed(),
        [] => {
            // Try parsing number from arbitrary first `font-[1100]``
            // If we parse then it's a size.
            // Otherwise it's a font-family.
            maybe_weight(arbitrary)
                .or_else(|_| maybe_size(arbitrary))
                .unwrap_or(TailwindFontFamily {}.boxed())
        }
        // TODO: This collision fails needs work?
        _ => TailwindFontFamily {}.boxed(),
    };
    Ok(out)
}

fn maybe_weight(arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
    let w = arbitrary.as_integer()?;
    Ok(TailwindFontWeight::new(w).boxed())
}

fn maybe_size(arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
    let w = arbitrary.as_integer()?;
    Ok(TailwindFontWeight::new(w).boxed())
}
