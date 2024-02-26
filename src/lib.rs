#[macro_export]
macro_rules! tw {
    ($($item:expr),+ $(,)?) => {{
        let mut result = String::new();
        $(
            // Long lived expressions.
            let item = $item;
            let tailwind_class = item.to_tailwind_class();
            if let Some(class) = tailwind_class {
                let class = class.trim();
                if !class.is_empty() {
                    if !result.is_empty() { result.push(' '); }
                    result.push_str(class);
                }
            }
        )*
        result
    }};
}

trait ToTailwindClass<'a> {
    fn to_tailwind_class(&'a self) -> Option<&'a str>;
}

impl<'a> ToTailwindClass<'a> for String {
    fn to_tailwind_class(&'a self) -> Option<&'a str> {
        Some(self.as_str())
    }
}

impl<'a> ToTailwindClass<'a> for str {
    fn to_tailwind_class(&'a self) -> Option<&'a str> {
        Some(self)
    }
}

impl<'a> ToTailwindClass<'a> for Option<String> {
    fn to_tailwind_class(&'a self) -> Option<&'a str> {
        self.as_deref()
    }
}

impl<'a> ToTailwindClass<'a> for Option<&'a str> {
    fn to_tailwind_class(&'a self) -> Option<&'a str> {
        *self
    }
}

#[test]
fn test_tw() {
    assert_eq!(tw!("hello"), "hello");
    assert_eq!(tw!("one", "two"), "one two")
}

#[test]
fn test_option() {
    let is_hovered = false;
    let is_hovered_class = Some("ring").filter(|_| is_hovered);

    let classes = tw!(
        "text-sm",
        Some("font-bold"),
        is_hovered_class,
        Some("ring").filter(|_| false),
        None::<String>,
        "bg-white",
        Some(" ")
    );
    assert_eq!(classes, "text-sm font-bold bg-white");
}
