pub trait ToTailwindClass {
    fn to_class(&self) -> String;
    fn with_class(&self, class: &str) -> String;
}
