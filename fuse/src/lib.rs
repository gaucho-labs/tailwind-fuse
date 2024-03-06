mod join;

mod merge;

pub use merge::*;

/// Used to Fuse Tailwind Classes together.
pub trait TailwindFuse {
    /// Strings are not guaranteed to be single class nor free of whitespace.
    fn fuse_classes(&self, class: &[&str]) -> String;
}

/// Will merge tailwind classes and handle conflicts using [`tw_merge()`]
pub struct TailwindMerge;

impl TailwindFuse for TailwindMerge {
    fn fuse_classes(&self, class: &[&str]) -> String {
        tw_merge_slice(class)
    }
}

/// Will simply join tailwind classes together without handling conflicts
pub struct TaiwindJoin;

impl TailwindFuse for TaiwindJoin {
    fn fuse_classes(&self, class: &[&str]) -> String {
        class
            .iter()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .fold(String::new(), |mut acc, s| {
                if !acc.is_empty() {
                    acc.push(' ');
                }
                acc.push_str(s);
                acc
            })
    }
}

/// Used to extract a &str from a type
///
/// Implement this trait for your type to use it with the [`tw_join!`] and [`tw_merge!`] macros
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
