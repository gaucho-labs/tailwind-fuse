use super::*;

#[derive(Clone, Debug)]
pub struct TailwindRight {
    kind: UnitValue,
}

impl TailwindInstance for TailwindRight {
    fn collision_id(&self) -> &'static str {
        "right"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindRight {
    /// <https://tailwindcss.com/docs/top-right-bottom-left>
    pub fn parse(
        pattern: &[&str],
        arbitrary: &TailwindArbitrary,
        negative: Negative,
    ) -> Result<Self> {
        let kind = get_kind_px_full_auto_fact("right", pattern, arbitrary, negative)?;
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/right#syntax>
    pub fn check_valid(mode: &str) -> bool {
        check_valid_auto(mode)
    }
}
