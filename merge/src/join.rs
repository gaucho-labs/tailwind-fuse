#[macro_export]
macro_rules! tw_join {
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

#[test]
fn test_tw() {
    use crate::*;

    assert_eq!(tw_join!("hello"), "hello");
    assert_eq!(tw_join!("one", "two"), "one two")
}

#[test]
fn test_option() {
    use crate::*;

    let is_hovered = false;
    let is_hovered_class = Some("ring").filter(|_| is_hovered);

    let classes = tw_join!(
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
