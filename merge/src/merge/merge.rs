use std::collections::HashSet;

use ast::AstStyle;

use crate::merge::conflict::get_conflicts;

use crate::merge::parser::parse;

#[macro_export]
macro_rules! tw_merge {
    ($($item:expr),+ $(,)?) => {{
        let joined = $crate::tw_join!($($item),+);
        $crate::tw_merge(joined.as_str())
    }};
}

pub fn tw_merge(class: &str) -> String {
    let styles: Vec<Result<AstStyle, &str>> = ast::parse_tailwind(class);

    let mut valid_styles: Vec<Result<AstStyle, &str>> = vec![];
    let mut collision_styles: HashSet<Collision> = HashSet::new();

    // println!("styles: {styles:?}");

    for style in styles.into_iter().rev() {
        let style = match style {
            Ok(style) => style,
            Err(_) => continue,
        };

        let elements = style.elements.as_slice();
        let result = parse(elements, style.arbitrary.unwrap_or_default());

        match result {
            Err(error) => {
                #[cfg(debug_assertions)]
                println!("No Instance found: {style:?} {error:?}");
                valid_styles.push(Ok(style));
            }
            Ok(collision_id) => {
                // println!("collision_id: {collision_id} {style:?}");
                // hover:md:focus
                let all_variants: Vec<&str> = style.variants.clone();

                let collision = Collision {
                    important: style.important,
                    variants: all_variants.clone(),
                    collision_id,
                };

                if collision_styles.contains(&collision) {
                    continue;
                }

                // Add the current collision_id.
                collision_styles.insert(collision);

                if let Some(collisions) = get_conflicts(collision_id) {
                    // println!("COLLISIONS {collisions:?}");
                    collisions.into_iter().for_each(|collision_id| {
                        let collision = Collision {
                            important: style.important,
                            variants: all_variants.clone(),
                            collision_id,
                        };

                        collision_styles.insert(collision);
                    });
                }

                // println!("pushing style: {}", style);
                valid_styles.push(Ok(style));
            }
        }
    }

    valid_styles.reverse();

    valid_styles
        .into_iter()
        .map(|s| match s {
            Ok(style) => style.source,
            Err(s) => s,
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Collision<'a> {
    important: bool,
    variants: Vec<&'a str>,
    collision_id: &'a str,
}
