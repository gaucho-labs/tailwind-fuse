mod conflict;
mod merge_impl;
mod parser;

pub use merge_impl::tw_merge_with_options;

#[macro_export]
macro_rules! tw_merge {
    ($($item:expr),+ $(,)?) => {{
        let joined = $crate::tw_join!($($item),+);
        $crate::tw_merge(joined.as_str())
    }};
}

/// Merges
pub fn tw_merge(class: &str) -> String {
    tw_merge_with_options(
        Default::default(),
        noop_collision_id_fn,
        noop_collision_fn,
        class,
    )
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MergeOptions {
    /// Custom prefix for modifiers in Tailwind classes
    /// Default is empty string
    /// https://tailwindcss.com/docs/configuration#prefix
    pub prefix: &'static str,
    /// Custom separator for modifiers in Tailwind classes
    /// Default is `:`
    /// https://tailwindcss.com/docs/configuration#separator
    pub separator: &'static str,
}

impl Default for MergeOptions {
    fn default() -> Self {
        DEFAULT_OPTIONS
    }
}

const DEFAULT_OPTIONS: MergeOptions = MergeOptions {
    prefix: "",
    separator: ":",
};

impl From<MergeOptions> for ast::AstParseOptions<'static> {
    fn from(options: MergeOptions) -> Self {
        ast::AstParseOptions {
            prefix: options.prefix,
            separator: options.separator,
        }
    }
}

/// Return a conflict id for given tailwind instruction.
pub trait CollisionIdFn {
    /// elements: parts of the tailwind class separated by `-`
    /// arbitrary: the arbitrary value at the end of the tailwind class
    fn apply(&self, elements: &[&str], arbitrary: Option<&str>) -> Option<&'static str>;
}

impl<F> CollisionIdFn for F
where
    F: Fn(&[&str], Option<&str>) -> Option<&'static str>,
{
    fn apply(&self, elements: &[&str], arbitrary: Option<&str>) -> Option<&'static str> {
        self(elements, arbitrary)
    }
}

/// Return list of collision ids for given collision id.
pub trait GetCollisionsFn {
    fn apply(&self, collision_id: &str) -> Option<Vec<&'static str>>;
}

impl<F> GetCollisionsFn for F
where
    F: Fn(&str) -> Option<Vec<&'static str>>,
{
    fn apply(&self, collision_id: &str) -> Option<Vec<&'static str>> {
        self(collision_id)
    }
}

fn noop_collision_id_fn(_: &[&str], _: Option<&str>) -> Option<&'static str> {
    None
}

fn noop_collision_fn(_: &str) -> Option<Vec<&'static str>> {
    None
}
