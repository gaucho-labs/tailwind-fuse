use std::collections::BTreeSet;

use tailwind_ast::AstStyle;
use tailwind_model::TailwindInstruction;

#[macro_export]
macro_rules! tw_merge {
    ($($item:expr),+ $(,)?) => {{
        let joined = $crate::tw_join!($($item),+);
        if let Some(result) = $crate::merge::tw_merge(joined.as_str()) {
            result
        } else {
            joined
        }
    }};
}

pub fn tw_merge(class: &str) -> Option<String> {
    let styles: Vec<AstStyle> = {
        let styles = tailwind_ast::parse_tailwind(class);
        if let Ok(styles) = styles {
            styles
        } else {
            return None;
        }
    };

    let mut valid_styles: Vec<AstStyle> = vec![];
    let mut seen_styles: BTreeSet<String> = BTreeSet::new();

    for style in styles.into_iter().rev() {
        let parse = TailwindInstruction::from(style.clone());
        let instance = parse.get_instance();
        match instance {
            Err(error) => {
                #[cfg(debug_assertions)]
                println!("No Instance found: {parse:?} {error:?}");
            }
            Ok(instance) => {
                let collision_id = instance.collision_id();
                if seen_styles.contains(&collision_id) {
                    continue;
                }

                let collisions = instance.get_collisions();
                let all_variants = style
                    .variants
                    .iter()
                    .flat_map(|v| v.names.iter().cloned())
                    .collect::<String>();

                collisions.into_iter().for_each(|collision| {
                    let collision = format!("{all_variants}{collision}");
                    seen_styles.insert(collision);
                });

                valid_styles.push(style);
            }
        }
    }

    let mut result = valid_styles
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    result.reverse();

    Some(result.join(" "))
}

#[cfg(test)]
mod test {
    use crate::merge::tw_merge;

    #[test]
    fn test_tw_merge() {
        let classes = tw_merge!("bg-red-500", "bg-blue-500", "text-green-500");
        assert_eq!(classes, "bg-blue-500 text-green-500")
    }

    #[test]
    fn test_conflict() {
        let class = "bg-red-500 bg-blue-500 text-green-500";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "bg-blue-500 text-green-500");

        let class = "bg-red-500 bg-blue-500 text-green-500 bg-amber-500";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "text-green-500 bg-amber-500");
    }

    #[test]
    fn test_conflict_with_modifiers() {
        let class = "bg-red-500 bg-blue-500 hover:bg-green-500 text-green-500";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "bg-blue-500 hover:bg-green-500 text-green-500");

        let class = "bg-red-500 bg-blue-500 hover:bg-green-500 text-green-500 hover:text-red-500";
        let result = tw_merge(class).unwrap();
        assert_eq!(
            result,
            "bg-blue-500 hover:bg-green-500 text-green-500 hover:text-red-500"
        );
    }

    #[test]
    fn test_conflict_with_arbitrary() {
        let class = "bg-red-500 bg-[#000000]";
        let result = tw_merge(class).unwrap();

        assert_eq!(result, "bg-[#000000]")
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

        let classes = tw_merge!("stroke-2", "stroke-[3px]");
        assert_eq!(classes, "stroke-[3px]");

        let classes = tw_merge!("stroke-black", "stroke-red-500", "stroke-blue-100");
        assert_eq!(classes, "stroke-blue-100");
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

    #[test]
    fn test_padding_narrowing() {
        let classes = tw_merge!("p-10", "px-5");
        assert_eq!(classes, "p-10 px-5");
        let classes = tw_merge!("px-5", "py-5", "p-10",);
        assert_eq!(classes, "p-10");
    }

    #[test]
    fn test_gap_narrowing() {
        let classes = tw_merge!("gap-10", "gap-x-5");
        assert_eq!(classes, "gap-10 gap-x-5");

        let classes = tw_merge!("gap-x-5", "gap-y-5", "gap-10");
        assert_eq!(classes, "gap-10");
    }
}
