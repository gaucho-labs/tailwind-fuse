pub mod cache;
mod join;

mod merge;

pub use merge::*;

pub trait TailwindClassMerger {
    fn merge_classes(&self, class: String) -> String;
}

pub struct DefaultTailwindClassMerge;

impl TailwindClassMerger for DefaultTailwindClassMerge {
    fn merge_classes(&self, class: String) -> String {
        tw_merge(class.as_str())
    }
}

pub struct NoopTailwindClassMerge;

impl TailwindClassMerger for NoopTailwindClassMerge {
    fn merge_classes(&self, class: String) -> String {
        class
    }
}

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
