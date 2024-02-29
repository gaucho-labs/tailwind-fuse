macro_rules! keyword_instance {
    ($t:ty => $a:literal) => {
        impl<T> From<T> for $t
        where
            T: Into<String>,
        {
            fn from(input: T) -> Self {
                Self { kind: StandardValue::from(input.into()) }
            }
        }
        impl TailwindInstance for $t {}
    };
}

macro_rules! color_instance {
    ($t:ty) => {
        impl<T> From<T> for $t
        where
            T: Into<TailwindColor>,
        {
            fn from(color: T) -> Self {
                Self { color: color.into() }
            }
        }
        impl $t {
            ///
            pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
                Ok(Self { color: TailwindColor::parse(input, arbitrary)? })
            }
            ///
            pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
                Ok(Self { color: TailwindColor::parse_arbitrary(arbitrary)? })
            }
        }
    };
}

pub(crate) use color_instance;
pub(crate) use keyword_instance;
