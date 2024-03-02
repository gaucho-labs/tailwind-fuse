use super::*;

#[derive(Clone, Debug)]
pub struct TailwindObjectFit {
    kind: StandardValue,
}

crate::macros::keyword_instance!(TailwindObjectFit => "object-fit");

impl TailwindObjectFit {
    /// <https://tailwindcss.com/docs/object-fit>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("object-fit", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/object-fit#syntax>
    pub fn check_valid(mode: &str) -> bool {
        [
            "contain",
            "cover",
            "fill",
            "inherit",
            "initial",
            "none",
            "revert",
            "scale-down",
            "unset",
        ]
        .contains(&mode)
    }
}
