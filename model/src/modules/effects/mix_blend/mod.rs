#[derive(Clone, Debug)]
pub struct TailwindBlend {
    kind: &'static str,
}

crate::macros::keyword_instance!(TailwindBlend => "mix-blend-mode",[
    "normal",
    "multiply",
    "screen",
    "overlay",
    "darken",
    "lighten",
    "color-dodge",
    "color-burn",
    "hard-light",
    "soft-light",
    "difference",
    "exclusion",
    "hue",
    "saturation",
    "color",
    "luminosity",
    "plus-lighter",
]);
