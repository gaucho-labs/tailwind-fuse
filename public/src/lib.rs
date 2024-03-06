#![forbid(missing_docs)]
#![doc = include_str!("../../README.md")]

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
/// use tw_variant_macro::{TwVariant, TwClass};
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
pub use tw_variant_macro::TwClass;

/// Represents a customizable property (variant) of a UI element.
/// Each variant must be an enum with a default case.
///
/// Use `.to_class()` to get the class for the variant and `.with_class()` to append a class.
///
/// # Example
///
/// ```rust
/// use tailwind_fuse::*;
/// use tw_variant_macro::TwVariant;
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
pub use tw_variant_macro::TwVariant;

/// A trait to convert a type into a Tailwind class.
/// Implemented automatically for usages of [`TwClass`] and [`TwVariant`].
#[cfg(feature = "variant")]
pub use tw_utils::IntoTailwindClass;

pub use tw_fuse::*;
