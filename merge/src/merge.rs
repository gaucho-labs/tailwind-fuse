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
