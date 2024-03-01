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

impl Display for TailwindOutlineStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "outline")?;
        match &self.kind {
            StandardValue::Keyword(s) => match s.as_str() {
                "solid" => write!(f, ""),
                "<NONE>" => write!(f, "-none"),
                s @ ("dashed" | "dotted" | "double" | "hidden") => write!(f, "-{}", s),
                _ => write!(f, "-style-{}", s),
            },
            StandardValue::Arbitrary(a) => a.write_class(f, "-style-"),
        }
    }
}

impl TailwindInstance for TailwindOutlineStyle {
    fn collision_id(&self) -> String {
        "outline-style".into()
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
