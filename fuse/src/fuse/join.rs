/// Joins the given classes into a single string.
///
/// Items can be of type &[`str`], [`String`], [`Option<&str>`] or [`Option<String>`].
///
/// If you want to handle conflicts use [`crate::tw_merge!`].
///
/// If you want a custom type to be used with this macro, implement the [`crate::MaybeIntoTailwindClass`] trait.
#[macro_export]
macro_rules! tw_join {
     ($item:expr) => {{
        use $crate::MaybeIntoTailwindClass;
        let tailwind_class = $item.to_tailwind_class();
        if let Some(class) = tailwind_class {
            class.trim().into()
        } else {
            String::new()
        }
    }};
    ($($item:expr),+ $(,)?) => {{
        use $crate::MaybeIntoTailwindClass;
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
    let classes = tw_join!(
        "text-sm",
        Some("font-bold"),
        Some("ring").filter(|_| false),
        None::<String>,
        "bg-white",
        Some(" "),
        "".to_string(),
        "italic text-green-500"
    );
    assert_eq!(classes, "text-sm font-bold bg-white italic text-green-500");
}
