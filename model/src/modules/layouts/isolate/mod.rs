use super::*;

#[derive(Clone, Debug)]
pub struct TailwindIsolation {
    kind: StandardValue,
}

crate::macros::keyword_instance!(TailwindIsolation => "isolation");

impl TailwindIsolation {
    /// https://tailwindcss.com/docs/isolation
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("isolate", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/isolation#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            // Keyword values
            "auto", "isolate", // Global values
            "inherit", "initial", "revert", "unset",
        ]);
        set.contains(mode)
    }
}
