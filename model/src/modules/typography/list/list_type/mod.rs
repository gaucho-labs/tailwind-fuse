// TODO: This is wrong, can contain arbitrary values.
#[derive(Debug, Clone)]
pub struct TailwindListStyle {
    kind: &'static str,
}

// In order to take care of user tailwind overrides, supply all possible css values.
crate::macros::keyword_instance!(TailwindListStyle => "list-style-type", [
    // default tailwind values.
    "none", "disc", "decimal",
    // Other possible values.
    "circle", "georgian", "inherit", "initial",
    "kannada", "square", "trad-chinese-informal", "unset",
]);
