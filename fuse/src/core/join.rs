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
        use $crate::AsTailwindClass;
        let tailwind_class = $item.as_tailwind_class();
        tailwind_class.trim().to_string()
    }};
    ($($item:expr),+ $(,)?) => {{
        use $crate::AsTailwindClass;
        let mut result = String::new();
        $(
            // Long lived expressions.
            let class = $item;
            let class = class.as_tailwind_class();
            let class = class.trim();
            if !class.is_empty() {
                if !result.is_empty() { result.push(' '); }
                result.push_str(class);
            }
        )*
        result
    }};
}

#[test]
fn test_tw() {
    assert_eq!(tw_join!("one"), "one");
    assert_eq!(tw_join!("one", "two"), "one two");
    assert_eq!(
        tw_join!(" one", "two ", " three".to_string()),
        "one two three"
    );
}
