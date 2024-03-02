use super::*;

impl From<&str> for StandardValue {
    fn from(kind: &str) -> Self {
        Self::Keyword(kind.into())
    }
}

impl From<String> for StandardValue {
    fn from(kind: String) -> Self {
        Self::Keyword(kind)
    }
}
