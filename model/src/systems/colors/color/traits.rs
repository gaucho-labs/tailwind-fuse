use super::*;

impl From<&str> for TailwindColor {
    fn from(s: &str) -> Self {
        Self::Keyword(s.to_string())
    }
}

impl From<String> for TailwindColor {
    fn from(s: String) -> Self {
        Self::Keyword(s)
    }
}
