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
        if let Ok(styles) = tailwind_ast::parse_tailwind(class) {
            styles
        } else {
            return None;
        }
    };

    let mut valid_styles: Vec<AstStyle> = vec![];
    let mut collision_styles: BTreeSet<String> = BTreeSet::new();

    for style in styles.into_iter().rev() {
        let parse = TailwindInstruction::from(style.clone());
        match parse.get_instance() {
            Err(error) => {
                #[cfg(debug_assertions)]
                println!("No Instance found: {parse:?} {error:?}");
                valid_styles.push(style);
            }
            Ok(instance) => {
                let collision_id = instance.collision_id();

                // hover:md:focus
                let all_variants: String = {
                    let mut all_variants = style
                        .variants
                        .iter()
                        .flat_map(|v| v.names.iter().cloned())
                        .collect::<Vec<_>>();
                    all_variants.sort();
                    all_variants.join(":")
                };

                let collision_id = format!("{all_variants}{collision_id}");
                if collision_styles.contains(&collision_id) {
                    continue;
                }

                let collisions = instance.get_collisions();
                println!("collisions {:?}", collisions);

                collisions.into_iter().for_each(|collision| {
                    let collision = format!("{all_variants}{collision}");
                    collision_styles.insert(collision);
                });

                println!("pushing style: {}", style);
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
    use std::result;

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

        let class = "bg-orange-300 bg-[#f5f5f5] text-green-400 text-orange-300";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "bg-[#f5f5f5] text-orange-300");

        let class = "bg-none bg-black";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "bg-black");
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

        let class = "hover:scale-1.5 hover:bg-orange-300";
        let result = tw_merge(class).unwrap();
        assert_eq!(&result, class)
    }

    #[test]
    fn test_conflict_with_arbitrary_properties() {
        let class = "bg-red-500 bg-[#000000]";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "bg-[#000000]");

        let class = "stroke-[hsl(340_80%_0%)] stroke-[10px] border-2";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "stroke-[hsl(340_80%_0%)] stroke-[10px] border-2");

        let class = "[paint-order:markers] [paint-order:normal]";
        let result = tw_merge(class).unwrap();
        // assert_eq!(result, "[paint-order:normal]"); todo fix! cancels each other out!

        let class = "[paint-order:markers] [--my-var:2rem] [paint-order:normal] [--my-var:4px]";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "[paint-order:normal] [--my-var:4px]");

        let class = "[paint-order;:markers] hover:[paint-order:normal]";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "[paint-order:markers] hover:[paint-order:normal]");

        let class = "hover;:[paint-order:markers] hover:[paint-order:normal]";
        let result = tw_merge(class).unwrap();
        assert_eq!("hover:[paint-order:normal]", result);

        let class = "hover;:focus:[paint-order:markers] focus:hover:[paint-order:normal]";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "focus:hover:[paint-order:normal]");

        let class = "[paint-order:markers] [paint-order:normal] [--my-var:2rem] lg:[--my-var:4px]";
        let result = tw_merge(class).unwrap();
        assert_eq!(
            result,
            "[paint-order:normal] [--my-var:2rem] lg:[--my-var:4px]"
        );

        let class = "[-unknown-prop:::123:::] [-unknown-prop:url(https://hicom)]";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "[-unknown-prop:url(https://hicom)]");

        let class = "![some:prop] [some:other] [some:one] ![some:another]";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "[some:one] ![some:another]");
    }

    #[test]
    fn test_conflict_with_arbitrary_values() {
        let class = "m-[2px] m-[10px]";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "m-[10px]");

        let class = "m-[2px] m-[11svmin] m-[12in] m-[13lvi] m-[14vb] m-[15vmax] m-[16mm] m-[17%] m-[18em] m-[19px] m-[10dvh]";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "m-[10dvh]");

        let class = "h-[10px] h-[11cqw] h-[12cqh] h-[13cqi] h-[14cqb] h-[15cqmin] h-[16cqmax]";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "h-[16cqmax]");

        let class = "z-20; z-[99]";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "z-[99]");

        let class = "my-[2px]; m-[10rem]";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "m-[10rem]");

        let class = "cursor-pointer; cursor-[grab]";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "cursor-[grab]");

        let class = "m-[2px]; m-[calc(100%-var(--arbitrary))]";
        let result = tw_merge(class).unwrap();
        assert_eq!("m-[calc(100%-var(--arbitrary))]", result);

        let class = "m-[2px]; m-[length:var(--mystery-var)]";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "m-[length:var(--mystery-var)]");

        let class = "opacity-10; opacity-[0.025]";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "opacity-[0.025]");

        let class = "scale-75; scale-[1.7]";
        let result = tw_merge(class).unwrap();
        assert_eq!("scale-[1.7]", result);

        let class = "brightness-90; brightness-[1.75]";
        let result = tw_merge(class).unwrap();
        assert_eq!("brightness-[1.75]", result);

        // Handling of value `0`
        let class = "min-h-[0.5px]; min-h-[0]";
        let result = tw_merge(class).unwrap();
        assert_eq!("min-h-[0]", result);

        let class = "text-[0.5px]; text-[color:0]";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "text-[0.5px] text-[color:0]");

        let class = "text-[0.5px]; text-[--my-0]";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "text-[0.5px] text-[--my-0]");

        let class = "hover:m-[2px] hover:m-[length:var(--c)]";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "hover:m-[length:var(--c)]");

        let class = "hover;:focus:m-[2px] focus:hover:m-[length:var(--c)]";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "focus:hover:m-[length:var(--c)]");
            
        let class = "border-b; border-[color:rgb(var(--color-gray-500-rgb)/50%))]";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "border-b border-[color:rgb(var(--color-gray-500-rgb)/50%))]");


        let class = "border-[color;:rgb(var(--color-gray-500-rgb)/50%))] border-b";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "border-[color:rgb(var(--color-gray-500-rgb)/50%))] border-b");


        let class = "border-b border-[color:rgb(var(--color-gray-500-rgb)/50%))] border-some-coloooor";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "border-b border-some-coloooor");

        let class = "grid-rows-[1fr,auto]; grid-rows-2";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "grid-rows-2");


        let class = "grid-rows-[repeat(20,minmax(0,1fr))]; grid-rows-3";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "grid-rows-3");

        let class = "mt-2 mt-[calc(theme(fontSize.4xl)/1.125)]";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "mt-[calc(theme(fontSize.4xl)/1.125)]");

        let class = "p-2 p-[calc(theme(fontSize.4xl)/1.125)_10px]";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "p-[calc(theme(fontSize.4xl)/1.125)_10px]");

        let class = "mt-2; mt-[length:theme(someScale.someValue)]";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "mt-[length:theme(someScale.someValue)]");

        let class = "mt-2 mt-[theme(someScale.someValue)]" ;
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "mt-[theme(someScale.someValue)]");

        let class = "text-2xl text-[length:theme(someScale.someValue)]";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "text-[length:theme(someScale.someValue)]");

        let class ="text-2xl text-[calc(theme(fontSize.4xl)/1.125)]";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "text-[calc(theme(fontSize.4xl)/1.125)]");

        let class = "bg-cover bg-[percentage:30%] bg-[length:200px_100px]";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "bg-[length:200px_100px]");


        let class = "bg-none bg-[url(.)] bg-[image:.] bg-[url:.] bg-[linear-gradient(.)] bg-gradient-to-r";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "bg-gradient-to-r");
    }
}

#[cfg(test)]
mod hardmode {
    use crate::merge::tw_merge;
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

    #[test]
    fn merges_classes_from_same_group_correctly() {
        let class = "overflow-x-auto overflow-x-hidden";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "overflow-x-hidden");

        let class = "basis-full basis-auto";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "basis-auto");

        let class = "w-full w-fit";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "w-fit");

        let class = "overflow-x-auto overflow-x-hidden overflow-x-scroll";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "overflow-x-scroll");

        let class = "overflow-x-auto hover:overflow-x-hidden overflow-x-scroll";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "hover:overflow-x-hidden overflow-x-scroll");

        let class =
            "overflow-x-auto hover:overflow-x-hidden hover:overflow-x-auto overflow-x-scroll";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "hover:overflow-x-auto overflow-x-scroll");

        let class = "col-span-1 col-span-full";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "col-span-full");
    }

    #[test]
    fn merges_classes_from_font_variant_numeric_section_correctly() {
        let class = "lining-nums tabular-nums diagonal-fractions";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "lining-nums tabular-nums diagonal-fractions");

        let class = "normal-nums tabular-nums diagonal-fractions";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "tabular-nums diagonal-fractions");

        let class = "tabular-nums diagonal-fractions normal-nums";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "normal-nums");

        let class = "tabular-nums proportional-nums";
        let result = tw_merge(class).unwrap();
        assert_eq!(result, "proportional-nums");
    }
}

#[test]
fn test_merge_with_non_tailwind() {
    let class_names: String = vec![
        "header",
        "nav-bar",
        "feature",
        // tailwind class
        "flex-row",
        "section",
        "container-fluid",
        "row",
        "column",
        // tailwind class
        "flex-col",
    ]
    .into_iter()
    .map(String::from)
    .collect::<Vec<_>>()
    .join(" ");

    let result = tw_merge(&class_names).unwrap();

    // flex row should be removed in favor of flex-col.
    let expected = class_names.replace(" flex-row", "");

    assert_eq!(result, expected);
}
