use super::*;

#[derive(Clone, Debug)]
pub struct TailwindIsolation {
    kind: &'static str,
}
// https://tailwindcss.com/docs/isolation

crate::macros::keyword_instance!(TailwindIsolation => "isolation", [
    // Keyword values
    "isolate", "isolation-auto",
    // Global values
    "inherit", "initial", "revert", "unset",
]);
