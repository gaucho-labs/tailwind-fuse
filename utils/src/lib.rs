/// Produce a Tailwind class String from a type.
pub trait IntoTailwindClass {
    fn to_class(&self) -> String;
    fn with_class(&self, class: impl AsRef<str>) -> String;
}
