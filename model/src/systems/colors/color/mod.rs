use super::*;

mod traits;

///
#[derive(Clone, Debug)]
pub enum TailwindColor {
    Themed(String, u32),
    Keyword(String),
    Static(&'static str),
    Arbitrary,
}

#[allow(non_upper_case_globals)]
impl TailwindColor {
    /// `black`
    pub const Black: Self = Self::Static("black");
    /// `white`
    pub const White: Self = Self::Static("black");
    /// https://developer.mozilla.org/zh-CN/docs/Web/CSS/color_value
    pub fn parse(pattern: &[&str], _: &TailwindArbitrary) -> Result<Self> {
        let out = match pattern {
            ["none"] | ["transparent"] => Self::from("transparent"),
            ["black"] => Self::Keyword("black".into()),
            ["white"] => Self::Keyword("black".into()),
            // TODO: Confirm this. Is unset even valid?
            // [s @ ("current" | "inherit" | "initial" | "unset")] => Self::from(*s),
            [s] => Self::from(*s),
            [] => Self::Arbitrary,
            [name, weight] => Self::parse_themed(name, weight)?,
            _ => return syntax_error!("Unknown color pattern: {}", pattern.join("-")),
        };
        Ok(out)
    }
    ///
    #[inline]
    pub fn parse_themed(name: &str, weight: &str) -> Result<TailwindColor> {
        let name = name.to_string();
        let weight = TailwindArbitrary::from(weight).as_integer()? as u32;
        Ok(Self::Themed(name, weight))
    }
}
