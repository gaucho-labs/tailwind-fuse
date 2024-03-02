use crate::StandardValue;

use super::*;

#[derive(Clone, Debug)]
pub struct TailwindOutlineStyle {
    kind: StandardValue,
}

impl<T> From<T> for TailwindOutlineStyle
where
    T: Into<String>,
{
    fn from(input: T) -> Self {
        Self {
            kind: StandardValue::from(input.into()),
        }
    }
}

impl TailwindInstance for TailwindOutlineStyle {
    fn collision_id(&self) -> &'static str {
        "outline-style"
    }

    fn get_collisions(&self) -> Vec<&'static str> {
        vec![]
    }
}

impl TailwindOutlineStyle {
    /// <https://tailwindcss.com/docs/outline-style>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            ["none"] => StandardValue::from("<NONE>"),
            [] if arbitrary.is_none() => StandardValue::from("solid"),
            _ => StandardValue::parser("outline-style", &Self::check_valid)(pattern, arbitrary)?,
        };
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/outline-style#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "auto", "dashed", "dotted", "double", "groove", "inherit", "initial", "inset", "none",
            "outset", "revert", "ridge", "solid", "unset",
        ]);
        set.contains(mode)
    }
}
