use std::collections::HashSet;

use ast::AstStyle;

use crate::merge::conflict::get_collisions;
use crate::{CollisionIdFn, GetCollisionsFn, MergeOptions};

pub fn tw_merge_with_options(
    options: MergeOptions,
    collision_id_fn: impl CollisionIdFn,
    collisions_fn: impl GetCollisionsFn,
    class: &str,
) -> String {
    let styles: Vec<Result<AstStyle, &str>> = ast::parse_tailwind(class, options.into());

    let mut valid_styles: Vec<Result<AstStyle, &str>> = vec![];
    let mut collision_styles: HashSet<Collision> = HashSet::new();

    for style in styles.into_iter().rev() {
        let style = match style {
            Ok(style) => style,
            Err(s) => {
                valid_styles.push(Err(s));
                continue;
            }
        };

        let elements = style.elements.as_slice();
        let result = collision_id_fn
            .apply(elements, style.arbitrary)
            .map(Ok)
            .unwrap_or_else(|| {
                let arbitrary = style.arbitrary.unwrap_or_default();
                crate::merge::parser::parse(elements, arbitrary)
            });

        match result {
            Err(error) => {
                #[cfg(debug_assertions)]
                println!("No Instance found: {style:?} {error:?}");
                valid_styles.push(Ok(style));
            }
            Ok(collision_id) => {
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

                let collisions = collisions_fn
                    .apply(collision_id)
                    .or_else(|| get_collisions(collision_id));

                if let Some(collisions) = collisions {
                    collisions.into_iter().for_each(|collision_id| {
                        let collision = Collision {
                            important: style.important,
                            variants: all_variants.clone(),
                            collision_id,
                        };

                        collision_styles.insert(collision);
                    });
                }

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
