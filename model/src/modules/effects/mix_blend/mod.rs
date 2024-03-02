use super::*;

#[derive(Clone, Debug)]
pub struct TailwindBlend {
    kind: StandardValue,
}

crate::macros::keyword_instance!(TailwindBlend => "mix-blend-mode");

impl TailwindBlend {
    /// <https://tailwindcss.com/docs/mix-blend-mode>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("mix-blend", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// get class of `<blend-mode>`
    ///
    /// - https://developer.mozilla.org/zh-CN/docs/Web/CSS/blend-mode

    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
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
        set.contains(mode)
    }
}
