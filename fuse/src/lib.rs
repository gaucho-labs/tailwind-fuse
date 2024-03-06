mod join;

mod merge;

pub use merge::*;

/// Used to Fuse Tailwind Classes together.
pub trait TailwindClassFuse {
    fn fuse_classes(&self, class: Vec<&str>) -> String;

    fn concat(class: &[&str]) -> String {
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

/// Will merge tailwind classes and handle conflicts using [`tw_merge`]
pub struct DefaultTailwindClassMerge;

impl TailwindClassFuse for DefaultTailwindClassMerge {
    fn fuse_classes(&self, class: Vec<&str>) -> String {
        let class = Self::concat(class.as_slice());
        tw_merge(class.as_str())
    }
}

/// Will simply join tailwind classes together without handling conflicts
pub struct TailwindClassJoin;

impl TailwindClassFuse for TailwindClassJoin {
    fn fuse_classes(&self, class: Vec<&str>) -> String {
        Self::concat(class.as_slice())
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
