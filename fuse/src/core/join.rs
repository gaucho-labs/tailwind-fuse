/// Joins the given classes into a single string.
///
/// Items can be anything that implements [`crate::AsTailwindClass`].
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
        let a = $a;
        let a_class = a.as_class().trim();
        let b = $b;
        let b_class = b.as_class().trim();
        format!(
            "{}{}{}",
            a_class,
            if b_class.is_empty() { "" } else { " " },
            b_class
        )
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        use $crate::AsTailwindClass;
        let a = $a;
        let a_class = a.as_class().trim();
        let b = $b;
        let b_class = b.as_class().trim();
        let c = $c;
        let c_class = c.as_class().trim();
        format!(
            "{}{}{}{}{}",
            a_class,
            if b_class.is_empty() { "" } else { " " },
            b_class,
            if c_class.is_empty() { "" } else { " " },
            c_class
        )
    }};
    ($a:expr, $b:expr, $c:expr, $d:expr) => {{
        use $crate::AsTailwindClass;
        let a = $a;
        let a_class = a.as_class().trim();
        let b = $b;
        let b_class = b.as_class().trim();
        let c = $c;
        let c_class = c.as_class().trim();
        let d = $d;
        let d_class = d.as_class().trim();
        format!(
            "{}{}{}{}{}{}{}",
            a_class,
            if b_class.is_empty() { "" } else { " " },
            b_class,
            if c_class.is_empty() { "" } else { " " },
            c_class,
            if d_class.is_empty() { "" } else { " " },
            d_class
        )
    }};
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {{
        use $crate::AsTailwindClass;
        let a = $a;
        let a_class = a.as_class().trim();
        let b = $b;
        let b_class = b.as_class().trim();
        let c = $c;
        let c_class = c.as_class().trim();
        let d = $d;
        let d_class = d.as_class().trim();
        let e = $e;
        let e_class = e.as_class().trim();
        format!(
            "{}{}{}{}{}{}{}{}{}",
            a_class,
            if b_class.is_empty() { "" } else { " " },
            b_class,
            if c_class.is_empty() { "" } else { " " },
            c_class,
            if d_class.is_empty() { "" } else { " " },
            d_class,
            if e_class.is_empty() { "" } else { " " },
            e_class
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
fn join() {
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
    assert_eq!(tw_join!("a", "    ", "b", "c", " "), "a b c");

    assert_eq!(
        tw_join!("a", (false).then_some("b"), (true).then_some("c")),
        "a c"
    )
}
