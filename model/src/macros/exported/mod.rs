#[macro_export]
macro_rules! syntax_error {
    ($msg:literal $(,)?) => {
        Err(crate::error::TailwindError::syntax_error($msg.to_string()))
    };
    ($fmt:expr, $($arg:tt)*) => {
        Err(crate::error::TailwindError::syntax_error(format!($fmt, $($arg)*)))
    };
}
