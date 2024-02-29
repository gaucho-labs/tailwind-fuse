use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindRight {
    kind: UnitValue,
}

impl Display for TailwindRight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.write_negative(f)?;
        self.kind.write_class(f, "right-")
    }
}

impl TailwindInstance for TailwindRight {
}

impl TailwindRight {
    /// <https://tailwindcss.com/docs/top-right-bottom-left>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Self> {
        let kind = get_kind_px_full_auto_fact("right", pattern, arbitrary, negative)?;
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/right#syntax>
    pub fn check_valid(mode: &str) -> bool {
        check_valid_auto(mode)
    }
}
