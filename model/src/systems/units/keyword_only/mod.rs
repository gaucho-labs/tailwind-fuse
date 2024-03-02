use super::*;

mod traits;

/// Used to represent those attributes that only have keywords
#[derive(Debug, Clone)]
pub enum StandardValue {
    Keyword(String),
    Arbitrary,
}

impl StandardValue {
    pub fn parser(
        id: &'static str,
        check_valid: &'static impl Fn(&str) -> bool,
    ) -> impl Fn(&[&str], &TailwindArbitrary) -> Result<Self> {
        move |pattern: &[&str], arbitrary: &TailwindArbitrary| match pattern {
            [] => Self::parse_arbitrary(arbitrary),
            _ => Self::parse_keyword(pattern, id, check_valid),
        }
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Arbitrary)
    }
    pub fn parse_keyword(
        pattern: &[&str],
        id: &str,
        checker: &'static impl Fn(&str) -> bool,
    ) -> Result<Self> {
        let keyword = pattern.join("-");
        if !checker(&keyword) {
            return syntax_error!("{} does not a valid value of {}", keyword, id);
        }
        Ok(Self::Keyword(keyword))
    }
}
