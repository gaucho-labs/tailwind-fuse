use std::collections::HashSet;

use tailwind_ast::AstStyle;

use crate::merge::conflict::get_conflicts;

use crate::merge::parser::parse;

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
    let styles: Vec<AstStyle> = parse_classes(class)?;

    let mut valid_styles: Vec<AstStyle> = vec![];
    let mut collision_styles: HashSet<Collision> = HashSet::new();

    // println!("styles: {styles:?}");

    for style in styles.into_iter().rev() {
        let elements = remove_last_semicolon(style.elements.as_slice());
        let result = parse(elements.as_slice(), style.arbitrary.unwrap_or_default());

        match result {
            Err(error) => {
                #[cfg(debug_assertions)]
                println!("No Instance found: {style:?} {error:?}");
                valid_styles.push(style);
            }
            Ok(collision_id) => {
                // println!("collision_id: {collision_id}");
                // hover:md:focus
                let all_variants: Vec<&str> = {
                    let mut all_variants = style
                        .variants
                        .iter()
                        .flat_map(|v| v.names.iter().cloned())
                        .collect::<Vec<_>>();
                    all_variants.sort();
                    all_variants
                };

                let collision = Collision {
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
                            variants: all_variants.clone(),
                            collision_id,
                        };

                        collision_styles.insert(collision);
                    });
                }

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Collision<'a> {
    variants: Vec<&'a str>,
    collision_id: &'a str,
}

// Consider usage of semi-colon
fn parse_classes(class: &str) -> Option<Vec<AstStyle>> {
    let classes = class.split(';').map(str::trim).filter(|c| !c.is_empty());

    let mut styles = Vec::new();
    for class in classes {
        match tailwind_ast::parse_tailwind(class) {
            Ok(parsed_styles) => styles.extend(parsed_styles),
            Err(_) => return None,
        }
    }

    Some(styles)
}

fn remove_last_semicolon<'a>(elements: &'a [&'a str]) -> Vec<&'a str> {
    let mut elements = elements.to_vec();

    if let Some(last) = elements.last_mut() {
        if last.ends_with(';') {
            *last = &last[..last.len() - 1];
        }
    }
    elements
}
