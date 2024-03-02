use super::*;

#[derive(Clone, Debug)]
pub struct TailwindTop {
    kind: UnitValue,
}

// TODO: HOW DO THESE REFINE?
impl TailwindInstance for TailwindTop {
    fn collision_id(&self) -> &'static str {
        "top"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindTop {
    /// <https://tailwindcss.com/docs/top-right-bottom-left>
    pub fn parse(
        pattern: &[&str],
        arbitrary: &TailwindArbitrary,
        negative: Negative,
    ) -> Result<Self> {
        let kind = get_kind_px_full_auto_fact("top", pattern, arbitrary, negative)?;
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/top#syntax>
    pub fn check_valid(mode: &str) -> bool {
        check_valid_auto(mode)
    }
}
