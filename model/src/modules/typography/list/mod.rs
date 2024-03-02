use super::*;

pub(crate) mod list_position;
pub(crate) mod list_type;

pub(crate) fn list_adaptor(
    str: &[&str],
    arbitrary: &TailwindArbitrary,
) -> Result<Box<dyn TailwindInstance>> {
    let out = match str {
        // https://tailwindcss.com/docs/list-style-position
        [s @ ("inside" | "outside")] => TailwindListPosition::try_from(*s)?.boxed(),
        // https://tailwindcss.com/docs/list-style-type
        _ => TailwindListStyle::try_from(str)
            .or_else(|_| TailwindListStyle::try_from(arbitrary.as_str()))?
            .boxed(),
        // TODO: https://tailwindcss.com/docs/list-style-image
    };
    Ok(out)
}
