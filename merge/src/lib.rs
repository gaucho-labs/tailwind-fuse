pub mod cache;
mod join;
pub mod merge;

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
