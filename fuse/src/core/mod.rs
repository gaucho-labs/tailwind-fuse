pub(crate) mod join;

/// Merges all the Tailwind classes, resolving conflicts.
pub mod merge;

/// Used to extract a &str from a type
///
/// Implement this trait for your type to use it with the [`tw_join!`] and [`tw_merge!`] macros
pub trait AsTailwindClass {
    /// Extract a Tailwind class
    fn as_class(&self) -> &str;
}

impl<T> AsTailwindClass for T
where
    T: AsRef<str>,
{
    fn as_class(&self) -> &str {
        self.as_ref()
    }
}
