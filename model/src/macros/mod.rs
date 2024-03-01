// TODO: CONFIRM USAGES CAN HAVE THESE COLLISION IDS.
macro_rules! keyword_instance {
    ($t:ty => $a:literal) => {
        impl<T> From<T> for $t
        where
            T: Into<String>,
        {
            fn from(input: T) -> Self {
                Self {
                    kind: StandardValue::from(input.into()),
                }
            }
        }
        impl TailwindInstance for $t {
            fn collision_id(&self) -> String {
                $a.to_string()
            }
            fn get_collisions(&self) -> Vec<String> {
                vec![self.collision_id()]
            }
        }
    };
}

macro_rules! color_instance {
    ($t:ty) => {
        impl<T> From<T> for $t
        where
            T: Into<TailwindColor>,
        {
            fn from(color: T) -> Self {
                Self {
                    color: color.into(),
                }
            }
        }
        impl $t {
            ///
            pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
                Ok(Self {
                    color: TailwindColor::parse(input, arbitrary)?,
                })
            }
            ///
            pub fn parse_arbitrary(arbitrary: TailwindArbitrary) -> Result<Self> {
                Ok(Self {
                    color: TailwindColor::Arbitrary(arbitrary),
                })
            }
        }
    };
}

#[macro_export]
macro_rules! syntax_error {
    ($msg:literal $(,)?) => {
        Err(crate::error::TailwindError::syntax_error($msg.to_string()))
    };
    ($fmt:expr, $($arg:tt)*) => {
        Err(crate::error::TailwindError::syntax_error(format!($fmt, $($arg)*)))
    };
}

pub(crate) use color_instance;
pub(crate) use keyword_instance;
