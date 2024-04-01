/// Joins the given classes into a single string.
///
/// Items can be of type &[`str`] or [`String`].
///
/// If you want to handle conflicts use [`crate::tw_merge!`].
///
/// If you want a custom type to be used with this macro, implement the [`crate::AsTailwindClass`] trait.
#[macro_export]
macro_rules! tw_join {
     ($item:expr) => {{
        use $crate::AsTailwindClass;
        let tailwind_class = $item.as_class();
        tailwind_class.trim().to_string()
    }};
    ($a:expr, $b:expr) => {{
        use $crate::AsTailwindClass;
        format!(
            "{} {}",
            $a.as_class().trim(),
            $b.as_class().trim()
        )
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        use $crate::AsTailwindClass;
        format!(
            "{} {} {}",
            $a.as_class().trim(),
            $b.as_class().trim(),
            $c.as_class().trim()
        )
    }};
    ($a:expr, $b:expr, $c:expr, $d:expr) => {{
        use $crate::AsTailwindClass;
        format!(
            "{} {} {} {}",
            $a.as_class().trim(),
            $b.as_class().trim(),
            $c.as_class().trim(),
            $d.as_class().trim()
        )
    }};
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {{
        use $crate::AsTailwindClass;
        format!(
            "{} {} {} {} {}",
            $a.as_class().trim(),
            $b.as_class().trim(),
            $c.as_class().trim(),
            $d.as_class().trim(),
            $e.as_class().trim()
        )
    }};
    ($($item:expr),+ $(,)?) => {{
        use $crate::AsTailwindClass;
        let mut result = String::new();
        $(
            // Long lived expressions.
            let class = $item;
            let class = class.as_class();
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
    assert_eq!(tw_join!("a"), "a");
    assert_eq!(tw_join!("a", "b"), "a b");
    assert_eq!(tw_join!("a", "b", "c"), "a b c");
    assert_eq!(tw_join!("a", "b", "c", "d"), "a b c d");
    assert_eq!(tw_join!("a", "b", "c", "d", "e"), "a b c d e");
    assert_eq!(tw_join!("a", "b", "c", "d", "e", "f"), "a b c d e f");

    assert_eq!(
        tw_join!(" one", "two ", " three".to_string()),
        "one two three"
    );
}
