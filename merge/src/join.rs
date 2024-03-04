#[macro_export]
macro_rules! tw_join {
    ($($item:expr),+ $(,)?) => {{
        use $crate::MaybeToTailwindClass;
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

#[test]
fn test_tw() {
    assert_eq!(tw_join!("hello"), "hello");
    assert_eq!(tw_join!("one", "two"), "one two")
}

#[test]
fn test_option() {
    let is_hovered = false;
    let is_hovered_class = Some("ring").filter(|_| is_hovered);

    let classes = tw_join!(
        "text-sm",
        Some("font-bold"),
        is_hovered_class,
        Some("ring").filter(|_| false),
        None::<String>,
        "bg-white",
        Some(" "),
        "italic text-green-500"
    );
    assert_eq!(classes, "text-sm font-bold bg-white italic text-green-500");
}
