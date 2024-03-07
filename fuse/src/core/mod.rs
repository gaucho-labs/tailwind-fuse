pub(crate) mod join;

/// Merges all the Tailwind classes, resolving conflicts.
pub mod merge;

/// Used to extract a &str from a type
///
/// Implement this trait for your type to use it with the [`tw_join!`] and [`tw_merge!`] macros
pub trait MaybeIntoTailwindClass<'a> {
    /// Attempt to extract a Tailwind class
    fn to_tailwind_class(&'a self) -> Option<&'a str>;
}

impl<'a> MaybeIntoTailwindClass<'a> for String {
    fn to_tailwind_class(&'a self) -> Option<&'a str> {
        Some(self.as_str())
    }
}

impl<'a> MaybeIntoTailwindClass<'a> for str {
    fn to_tailwind_class(&'a self) -> Option<&'a str> {
        Some(self)
    }
}

impl<'a> MaybeIntoTailwindClass<'a> for Option<String> {
    fn to_tailwind_class(&'a self) -> Option<&'a str> {
        self.as_deref()
    }
}

impl<'a> MaybeIntoTailwindClass<'a> for Option<&'a str> {
    fn to_tailwind_class(&'a self) -> Option<&'a str> {
        *self
    }
}
