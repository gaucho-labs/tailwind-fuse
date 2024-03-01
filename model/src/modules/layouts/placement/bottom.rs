use super::*;

#[derive(Clone, Debug)]
pub struct TailwindBottom {
    kind: UnitValue,
}

impl Display for TailwindBottom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.write_negative(f)?;
        self.kind.write_class(f, "bottom-")
    }
}

// TODO: NOT SURE
impl TailwindInstance for TailwindBottom {
    fn collision_id(&self) -> String {
        "bottom".into()
    }

    fn get_collisions(&self) -> Vec<String> {
        vec![self.collision_id()]
    }
}

impl TailwindBottom {
    /// <https://tailwindcss.com/docs/top-right-bottom-left>
    pub fn parse(
        pattern: &[&str],
        arbitrary: &TailwindArbitrary,
        negative: Negative,
    ) -> Result<Self> {
        let kind = get_kind_px_full_auto_fact("bottom", pattern, arbitrary, negative)?;
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/bottom#syntax>
    pub fn check_valid(mode: &str) -> bool {
        check_valid_auto(mode)
    }
}
