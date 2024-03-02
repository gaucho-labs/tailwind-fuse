use super::*;

impl From<LengthUnit> for UnitValue {
    fn from(n: LengthUnit) -> Self {
        Self::Length(n)
    }
}

impl From<&str> for UnitValue {
    fn from(s: &str) -> Self {
        Self::Keyword(s.to_string())
    }
}

impl From<String> for UnitValue {
    fn from(s: String) -> Self {
        Self::Keyword(s)
    }
}

impl<'a> From<TailwindArbitrary<'a>> for UnitValue {
    fn from(a: TailwindArbitrary) -> Self {
        Self::Arbitrary
    }
}

impl<'a> From<&TailwindArbitrary<'a>> for UnitValue {
    fn from(a: &TailwindArbitrary) -> Self {
        Self::Arbitrary
    }
}

impl From<i32> for UnitValue {
    fn from(n: i32) -> Self {
        Self::Number {
            n: n as f32,
            is_negative: n < 0,
        }
    }
}
