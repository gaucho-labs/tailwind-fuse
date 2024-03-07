#![forbid(missing_docs)]

//! # Tailwind Fuse
//! [<img alt="github" src="https://img.shields.io/badge/github-gaucho--labs/tailwind--fuse-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/gaucho-labs/tailwind-fuse)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/tailwind-fuse.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/tailwind-fuse)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-tailwind--fuse-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/tailwind-fuse)
//! [<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/gaucho-labs/tailwind-fuse/ci.yml?branch=main&style=for-the-badge" height="20">](https://github.com/gaucho-labs/tailwind-fuse/actions?query=branch%3Amain)
//!
//! Two main utils are included in this crate:
//!
//! 1. Fuse: Fuse multiple Tailwind classes, with optional conflict resolution.
//!     > Inspired by [Tailwind Merge](https://github.com/dcastil/tailwind-merge)
//! 2. Variants: Compose type-safe variant classes
//!     > Inspired by [Class Variance Authority](https://github.com/joe-bell/cva)
//!
//!
//! ## Installation
//!
//! Variants requires the `variants` feature to be enabled.
//!
//! #### With variants
//! ```bash
//! cargo add tailwind-fuse --features variants
//! ```
//!
//! #### Without variants
//! ```bash
//! cargo add tailwind-fuse
//! ```
//!
//! ## Usage: Fuse
//!
//! You can use [`tw_join!`] to join Tailwind classes, and [`tw_merge!`] to merge Tailwind Classes handling conflicts.
//!
//! ```
//! use tailwind_fuse::*;
//!
//! // No conflict resolution
//! // "flex items-center justify-center"
//! let joined_class = tw_join!("flex items-center", "justify-center");
//!
//! // You can use Option to handle conditional rendering
//! // You can pass in &str, String, Option<String>, or Option<&str>
//! // "text-sm font-bold"
//! let classes = tw_join!(
//!     "text-sm",
//!     Some("font-bold"),
//!     None::<String>,
//!     Some("ring").filter(|_| false),
//!     Some(" "),
//!     "".to_string(),
//! );
//!
//! // Conflict resolution
//! // Right most class takes precedence
//! // p-4
//! let merged_class = tw_merge!("py-2 px-4", "p-4");
//!
//! // Refinements are permitted
//! // p-4 py-2
//! let merged_class = tw_merge!("p-4", "py-2");
//! ```
//!
//! ## Usage: Variants
//!
//! Useful for building components with first class support for tailwind. By default, conflicts are merged using [`tw_merge()`].
//!
//! Each [`TwClass`] represents a UI element with customizable properties (property is a "variant"). Each variant is represented by a [`TwVariant`], which must be an enum with a default case.
//!
//! The merge order is, where the last class takes preferences:
//! 1. [`TwClass`] base class
//! 2. [`TwVariant`] base class
//! 3. [`TwVariant`] enum variant class
//! 4. Override class [`IntoTailwindClass::with_class`] on the struct or builder
//!
//! ```
//! use tailwind_fuse::*;
//!
//! // Your Component Type
//! #[derive(TwClass)]
//! // Optional base class
//! #[tw(class = "flex")]
//! struct Btn {
//!     size: BtnSize,
//!     color: BtnColor,
//! }
//!
//! // Variant for size
//! #[derive(TwVariant)]
//! enum BtnSize {
//!     #[tw(default, class = "h-9 px-4 py-2")]
//!     Default,
//!     #[tw(class = "h-8 px-3")]
//!     Sm,
//!     #[tw(class = "h-10 px-8")]
//!     Lg,
//! }
//!
//! // Variant for color
//! #[derive(TwVariant)]
//! enum BtnColor {
//!     #[tw(default, class = "bg-blue-500 text-blue-100")]
//!     Blue,
//!     #[tw(class = "bg-red-500 text-red-100")]
//!     Red,
//! }
//! ```
//!
//! You can now use the `Btn` struct to generate Tailwind classes, using builder syntax, or using the struct directly
//!
//! ### Struct Syntax
//! ```
//! # use tailwind_fuse::*;
//! # // Your Component Type
//! # #[derive(TwClass)]
//! # // Optional base class
//! # #[tw(class = "flex")]
//! # struct Btn {
//! #     size: BtnSize,
//! #     color: BtnColor,
//! # }
//! # // Variant for size
//! # #[derive(TwVariant)]
//! # enum BtnSize {
//! #     #[tw(default, class = "h-9 px-4 py-2")]
//! #     Default,
//! #     #[tw(class = "h-8 px-3")]
//! #     Sm,
//! #     #[tw(class = "h-10 px-8")]
//! #     Lg,
//! # }
//! # // Variant for color
//! # #[derive(TwVariant)]
//! # enum BtnColor {
//! #     #[tw(default, class = "bg-blue-500 text-blue-100")]
//! #     Blue,
//! #     #[tw(class = "bg-red-500 text-red-100")]
//! #     Red,
//! # }
//! let button = Btn {
//!     size: BtnSize::Default,
//!     color: BtnColor::Blue,
//! };
//! // h-9 px-4 py-2 bg-blue-500 text-blue-100
//! button.to_class();
//! // Conflicts are resolved.
//! // h-9 px-4 py-2 text-blue-100 bg-green-500
//! button.with_class("bg-green-500");
//! ```
//!
//! ### Builder Syntax
//! You access the builder using the `variants` method. Every variant that is not provided will be replaced with the default variant.
//!
//! ```
//! # use tailwind_fuse::*;
//! #
//! # #[derive(TwClass)]
//! # // Optional base class
//! # #[tw(class = "flex")]
//! # struct Btn {
//! #     size: BtnSize,
//! #     color: BtnColor,
//! # }
//! #
//! # // Variant for size
//! # #[derive(TwVariant)]
//! # enum BtnSize {
//! #     #[tw(default, class = "h-9 px-4 py-2")]
//! #     Default,
//! #     #[tw(class = "h-8 px-3")]
//! #     Sm,
//! #     #[tw(class = "h-10 px-8")]
//! #     Lg,
//! # }
//! #
//! # // Variant for color
//! # #[derive(TwVariant)]
//! # enum BtnColor {
//! #     #[tw(default, class = "bg-blue-500 text-blue-100")]
//! #     Blue,
//! #     #[tw(class = "bg-red-500 text-red-100")]
//! #     Red,
//! # }
//!
//! // h-8 px-3 bg-red-500 text-red-100
//! let class = Btn::variant()
//!     .size(BtnSize::Sm)
//!     .color(BtnColor::Red)
//!     .to_class();
//!
//! // h-8 px-3 text-red-100 bg-green-500
//! let class = Btn::variant()
//!     .size(BtnSize::Sm)
//!     .color(BtnColor::Red)
//!     .with_class("bg-green-500");
//! ```
//!
//! #### VSCode Intellisense
//!
//! You can enable autocompletion inside `#[tw()]` using the steps below:
//!
//! 1. [Install the "Tailwind CSS IntelliSense" Visual Studio Code extension](https://marketplace.visualstudio.com/items?itemName=bradlc.vscode-tailwindcss)
//!
//! 2. Add the following to your [`settings.json`](https://code.visualstudio.com/docs/getstarted/settings):
//!
//! ```json
//! {
//!   "tailwindCSS.experimental.classRegex": [
//!     ["#[tw\\\\([^\\]]*class\\s*=\\s*\"([^\"]*)\"\\)]", "\"([^\"]*)\""]
//!   ]
//! }
//! ```
//!

/// Derives a class for use with Tailwind CSS in Rust components.
///
/// Allows building components with first-class support for Tailwind.
///
/// Defaults to using [`crate::tw_merge()`] to resolve conflicts.
///
/// Resolves conflicts using the following merge order:
/// - [`TwClass`] base class
/// - [`TwVariant`] base class
/// - [`TwVariant`] enum variant class
/// - Override class with `with_class`
///
/// # Example
///
/// ```rust
/// use tailwind_fuse::*;
///
/// #[derive(TwClass, Debug)]
/// // Optional base class.
/// #[tw(class = "flex")]
/// struct Btn {
///     size: BtnSize,
///     color: BtnColor,
/// }
///
/// #[derive(TwVariant, Debug)]
/// enum BtnSize {
///     #[tw(default, class = "h-9 px-4 py-2")]
///     Default,
///     #[tw(class = "h-8 px-3")]
///     Sm,
///     #[tw(class = "h-10 px-8")]
///     Lg,
/// }
///
/// #[derive(TwVariant, Debug)]
/// enum BtnColor {
///     #[tw(default, class = "bg-blue-500 text-blue-100")]
///     Blue,
///     #[tw(class = "bg-red-500 text-red-100")]
///     Red,
/// }
///
/// let btn = Btn { size: BtnSize::Default, color: BtnColor::Blue };
/// assert_eq!(btn.to_class(), "flex h-9 px-4 py-2 bg-blue-500 text-blue-100");
///
/// let btn_variant = Btn::variant().color(BtnColor::Red).to_class();
/// assert_eq!(btn_variant, "flex h-9 px-4 py-2 bg-red-500 text-red-100");
/// ```
///
#[cfg(feature = "variant")]
pub use tailwind_fuse_macro::TwClass;

/// Represents a customizable property (variant) of a UI element.
/// Each variant must be an enum with a default case.
///
/// Use `.to_class()` to get the class for the variant and `.with_class()` to append a class.
///
/// # Example
///
/// ```rust
/// use tailwind_fuse::*;
///
/// #[derive(TwVariant, Debug)]
/// // Optional base class
/// #[tw(class = "hover:brightness-50")]
/// enum BtnColor {
///     #[tw(default, class = "bg-blue-500 text-blue-100")]
///     Default,
///     #[tw(class = "bg-red-500 text-red-100")]
///     Red,
/// }
///
/// assert_eq!(BtnColor::Default.to_class(), "hover:brightness-50 bg-blue-500 text-blue-100");
///
/// let red_with_class = BtnColor::Red.with_class("flex");
/// assert_eq!(red_with_class, "hover:brightness-50 bg-red-500 text-red-100 flex");
/// ```
///
#[cfg(feature = "variant")]
pub use tailwind_fuse_macro::TwVariant;

pub use crate::fuse::merge::merge_impl::*;
pub use crate::fuse::merge::*;
pub use crate::fuse::*;

mod ast;
mod fuse;

/// Used to Fuse Tailwind Classes together.
pub trait TailwindFuse {
    /// Strings are not guaranteed to be single class nor free of whitespace.
    fn fuse_classes(&self, class: &[&str]) -> String;
}

/// Will merge tailwind classes and handle conflicts using [`tw_merge()`]
pub struct TailwindMerge;

impl TailwindFuse for TailwindMerge {
    fn fuse_classes(&self, class: &[&str]) -> String {
        crate::fuse::merge::tw_merge_slice(class)
    }
}

/// Will simply join tailwind classes together without handling conflicts
pub struct TaiwindJoin;

impl TailwindFuse for TaiwindJoin {
    fn fuse_classes(&self, class: &[&str]) -> String {
        class
            .iter()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .fold(String::new(), |mut acc, s| {
                if !acc.is_empty() {
                    acc.push(' ');
                }
                acc.push_str(s);
                acc
            })
    }
}

/// A trait to convert a type into a Tailwind class.
/// Implemented automatically for usages of [`TwClass`] and [`TwVariant`].
#[cfg(feature = "variant")]
pub trait IntoTailwindClass {
    /// Convert the type into a Tailwind class.
    fn to_class(&self) -> String;
    /// Append to the class (with override precedence) and return the new class.
    fn with_class(&self, class: impl AsRef<str>) -> String;
}
