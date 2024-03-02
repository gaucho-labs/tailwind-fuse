use super::*;

pub(crate) mod snap_align;
pub(crate) mod snap_stop;
pub(crate) mod snap_type;

pub(crate) fn snap_adaptor(pattern: &[&str]) -> Result<Box<dyn TailwindInstance>> {
    let out = match pattern {
        // https://tailwindcss.com/docs/scroll-snap-align
        [s @ ("start" | "end" | "center")] => TailwindSnapAlign::try_from(*s)?.boxed(),
        ["align", rest @ ..] => TailwindSnapAlign::try_from(rest)?.boxed(),
        // https://tailwindcss.com/docs/scroll-snap-stop
        [s @ ("normal" | "always")] => TailwindSnapStop::try_from(*s)?.boxed(),
        ["stop", rest @ ..] => TailwindSnapStop::try_from(rest)?.boxed(),
        _ => TailwindSnapType::try_from(pattern)?.boxed(),
    };
    Ok(out)
}
