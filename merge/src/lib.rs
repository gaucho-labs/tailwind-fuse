mod join;

mod merge;

pub use merge::*;

/// Used to merge tailwind classes
pub trait TailwindClassMerger {
    fn merge_classes(&self, class: String) -> String;
}

/// Will merge tailwind classes and handle conflicts using [`tw_merge`]
pub struct DefaultTailwindClassMerge;

impl TailwindClassMerger for DefaultTailwindClassMerge {
    fn merge_classes(&self, class: String) -> String {
        tw_merge(class.as_str())
    }
}

/// Will not merge tailwind classes
pub struct NoopTailwindClassMerge;

impl TailwindClassMerger for NoopTailwindClassMerge {
    fn merge_classes(&self, class: String) -> String {
        class
    }
}

/// Used to extract a &str from a type
/// Implement this trait for your type to use it with the [`tw_join!`] macro
pub trait MaybeToTailwindClass<'a> {
    fn to_tailwind_class(&'a self) -> Option<&'a str>;
}

impl<'a> MaybeToTailwindClass<'a> for String {
    fn to_tailwind_class(&'a self) -> Option<&'a str> {
        Some(self.as_str())
    }
}

impl<'a> MaybeToTailwindClass<'a> for str {
    fn to_tailwind_class(&'a self) -> Option<&'a str> {
        Some(self)
    }
}

impl<'a> MaybeToTailwindClass<'a> for Option<String> {
    fn to_tailwind_class(&'a self) -> Option<&'a str> {
        self.as_deref()
    }
}

impl<'a> MaybeToTailwindClass<'a> for Option<&'a str> {
    fn to_tailwind_class(&'a self) -> Option<&'a str> {
        *self
    }
}
