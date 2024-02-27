use tailwind_ast::{parse_tailwind, ASTVariant, AstStyle};

#[macro_export]
macro_rules! tw_merge {
    ($($item:expr),+ $(,)?) => {{
        let joined = $crate::tw_join!($($item),+);
        if let Some(result) = $crate::merge::merge_conflicts(joined.as_str()) {
            result
        } else {
            joined
        }
    }};
}

pub fn merge_conflicts(class: &str) -> Option<String> {
    let styles: Vec<AstStyle> = {
        let styles = parse_tailwind(class);
        if let Ok(styles) = styles {
            styles
        } else {
            return None;
        }
    };

    let mut seen_styles: Vec<SeenStyle> = vec![];

    let mut result = styles
        .into_iter()
        .rev()
        .filter_map(|style| {
            println!("HERE {:?}", style);
            let first_element = *style.elements.first()?;
            let variants = style.variants.clone();
            let seen_style = SeenStyle {
                element: first_element,
                variants,
            };

            if seen_styles.contains(&seen_style) {
                None
            } else {
                seen_styles.push(seen_style);
                Some(style)
            }
        })
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    result.reverse();

    Some(result.join(" "))
}

#[derive(Debug, Clone, PartialEq)]
struct SeenStyle<'a> {
    element: &'a str,
    // TODO: remove this clone?
    variants: Vec<ASTVariant<'a>>,
}

#[cfg(test)]
mod test {
    use crate::{merge::*, *};

    #[test]
    fn test_tw_merge() {
        let classes = tw_merge!("bg-red-500", "bg-blue-500", "text-green-500");
        assert_eq!(classes, "bg-blue-500 text-green-500")
    }

    #[test]
    fn test_conflict() {
        let class = "bg-red-500 bg-blue-500 text-green-500";
        let result = merge_conflicts(class).unwrap();
        assert_eq!(result, "bg-blue-500 text-green-500");

        let class = "bg-red-500 bg-blue-500 text-green-500 bg-amber-500";
        let result = merge_conflicts(class).unwrap();
        assert_eq!(result, "text-green-500 bg-amber-500");
    }

    #[test]
    fn test_conflict_with_modifiers() {
        let class = "bg-red-500 bg-blue-500 hover:bg-green-500 text-green-500";
        let result = merge_conflicts(class).unwrap();
        assert_eq!(result, "bg-blue-500 hover:bg-green-500 text-green-500");

        let class = "bg-red-500 bg-blue-500 hover:bg-green-500 text-green-500 hover:text-red-500";
        let result = merge_conflicts(class).unwrap();
        assert_eq!(
            result,
            "bg-blue-500 hover:bg-green-500 text-green-500 hover:text-red-500"
        );
    }

    #[test]
    fn test_conflict_with_arbitrary() {
        let class = "bg-red-500 bg-[#11111]";
        let result = merge_conflicts(class).unwrap();

        assert_eq!(result, "bg-[#11111]")
    }
}

#[cfg(test)]
mod hardmode {
    use crate::*;

    #[test]
    fn test_tw_merge_mixed_blend() {
        let classes = tw_merge!("mix-blend-normal", "mix-blend-multiply");
        assert_eq!(classes, "mix-blend-multiply");
    }

    #[test]
    fn test_tw_merge_height() {
        let classes = tw_merge!("h-10", "h-min");
        assert_eq!(classes, "h-min");
    }

    #[test]
    fn test_tw_merge_stroke() {
        let classes = tw_merge!("stroke-black", "stroke-1");
        assert_eq!(classes, "stroke-black stroke-1");

        let classes = tw_merge!("stroke-2", "stroke-[3]");
        assert_eq!(classes, "stroke-[3]");
    }

    #[test]
    fn test_tw_merge_outline() {
        let classes = tw_merge!("outline-black", "outline-1");
        assert_eq!(classes, "outline-black outline-1");
    }

    #[test]
    fn test_tw_merge_grayscale() {
        let classes = tw_merge!("grayscale-0", "grayscale-[50%]");
        assert_eq!(classes, "grayscale-[50%]");
    }
}
