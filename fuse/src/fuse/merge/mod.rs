pub(crate) mod get_collisions;
pub(crate) mod merge_impl;
mod parser;
mod validators;

/// Merges all the tailwind classes, resolving conflicts.
///
/// Items can be of type &[`str`], [`String`], [`Option<&str>`] or [`Option<String>`].
///
/// If you DON'T want to handle conflicts use [`crate::tw_join!`].
///
/// If you want a custom type to be used with this macro, implement the [`crate::MaybeIntoTailwindClass`] trait.
#[macro_export]
macro_rules! tw_merge {
    ($($item:expr),+ $(,)?) => {{
        let joined = $crate::tw_join!($($item),+);
        $crate::tw_merge(joined.as_str())
    }};
}

/// Merges all the tailwind classes, resolving conflicts.
pub fn tw_merge(class: impl AsRef<str>) -> String {
    tw_merge_slice(&[class.as_ref()])
}

/// Merges all the tailwind classes, resolving conflicts, with the provided options.
///
/// ## Example: With Tailwind Prefix
///
/// ```
/// # use tailwind_fuse::*;
/// const OPTIONS: MergeOptions = MergeOptions {
///   prefix: "tw-",
///   separator: ":",
/// };
///
/// pub fn my_custom_tw_merge(class: impl AsRef<str>) -> String {
///    tw_merge_with_options(class, OPTIONS)
/// }
/// ```
pub fn tw_merge_with_options(class: impl AsRef<str>, options: MergeOptions) -> String {
    merge_impl::tw_merge_with_override(
        options,
        noop_collision_id_fn,
        noop_collision_fn,
        &[class.as_ref()],
    )
}

/// Merges all the tailwind classes, resolving conflicts.
pub fn tw_merge_slice(class: &[&str]) -> String {
    merge_impl::tw_merge_with_override(
        Default::default(),
        noop_collision_id_fn,
        noop_collision_fn,
        class,
    )
}

fn noop_collision_id_fn(_: &[&str], _: Option<&str>) -> Option<&'static str> {
    None
}

fn noop_collision_fn(_: &str) -> Option<Vec<&'static str>> {
    None
}

/// Configuration for merging tailwind classes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MergeOptions {
    /// Custom prefix for modifiers in Tailwind classes
    ///
    /// Default is empty string
    ///
    /// <https://tailwindcss.com/docs/configuration#prefix>
    pub prefix: &'static str,
    /// Custom separator for modifiers in Tailwind classes
    ///
    /// Default is `:`
    ///
    /// <https://tailwindcss.com/docs/configuration#separator>
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

impl From<MergeOptions> for crate::ast::AstParseOptions<'static> {
    fn from(options: MergeOptions) -> Self {
        crate::ast::AstParseOptions {
            prefix: options.prefix,
            separator: options.separator,
        }
    }
}

/// Return a ConflictId for a given Tailwind Class.
pub trait CollisionIdFn {
    /// elements: parts of the tailwind class separated by `-`.
    ///
    /// (e.g. `bg-red-500` would be `["bg", "red", "500"]`)
    ///
    /// arbitrary: the arbitrary value at the end of the tailwind class
    ///
    /// <https://tailwindcss.com/docs/adding-custom-styles#using-arbitrary-values>
    fn apply(&self, elements: &[&str], arbitrary: Option<&str>) -> Option<&'static str>;
}

impl<F> CollisionIdFn for F
where
    F: Fn(&[&str], Option<&str>) -> Option<&'static str> + 'static,
{
    fn apply(&self, elements: &[&str], arbitrary: Option<&str>) -> Option<&'static str> {
        self(elements, arbitrary)
    }
}

/// Return list of CollisionIds that collide with the given CollisionId.
///
/// The list does not need to contain the given CollisionId.
///
/// e.g. "flex-row" should probably collide with "flex-col"
pub trait GetCollisionsFn {
    /// Return list of CollisionIds that collide with the given CollisionId.
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
