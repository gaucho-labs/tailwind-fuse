use std::sync::OnceLock;

/// Configuration for merging Tailwind classes.
#[derive(Clone, Copy, Debug)]
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
        MERGE_OVERRIDE
            .get()
            .copied()
            .unwrap_or(DEFAULT_MERGE_OPTIONS)
    }
}

const DEFAULT_MERGE_OPTIONS: MergeOptions = MergeOptions {
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

pub(crate) static MERGE_OVERRIDE: OnceLock<MergeOptions> = OnceLock::new();

/// Set global options for merging Tailwind classes.
pub fn set_tw_options(options: MergeOptions) {
    let _ = MERGE_OVERRIDE.set(options);
}
