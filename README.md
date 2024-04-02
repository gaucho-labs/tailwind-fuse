<!-- cargo-rdme start -->

# Tailwind Fuse
[<img alt="github" src="https://img.shields.io/badge/github-gaucho--labs/tailwind--fuse-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/gaucho-labs/tailwind-fuse)
[<img alt="crates.io" src="https://img.shields.io/crates/v/tailwind-fuse.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/tailwind-fuse)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-tailwind--fuse-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/tailwind-fuse)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/gaucho-labs/tailwind-fuse/ci.yml?branch=main&style=for-the-badge" height="20">](https://github.com/gaucho-labs/tailwind-fuse/actions?query=branch%3Amain)

Two main utils are included in this crate:

1. Fuse: Fuse multiple Tailwind classes, with optional conflict resolution.
    > Inspired by [Tailwind Merge](https://github.com/dcastil/tailwind-merge)
2. Variants: Compose type-safe variant classes
    > Inspired by [Class Variance Authority](https://github.com/joe-bell/cva)


## Installation

Variants requires the `variant` feature to be enabled.

#### With variant
```bash
cargo add tailwind-fuse --features variant
```

#### Without variant
```bash
cargo add tailwind-fuse
```

## Usage: Fuse

You can use [`tw_join!`] to join Tailwind classes, and [`tw_merge!`] to merge Tailwind Classes handling conflicts.


You can use anything that implements [`AsRef<str>`] or [`AsTailwindClass`]

```rust
use tailwind_fuse::*;

// No conflict resolution
assert_eq!(
   "flex items-center justify-center",
   tw_join!("flex", "items-center", "justify-center")
);

// Conflict resolution
// Right most class takes precedence
assert_eq!("p-4", tw_merge!("py-2 px-4", "p-4"));

// Refinements are permitted
assert_eq!("p-4 py-2", tw_merge!("p-4", "py-2"));
```

## Usage: Variants

Useful for building components with first class support for tailwind. By default, conflicts are merged using [`tw_merge()`].

Each [`TwClass`] represents a UI element with customizable properties (property is a "variant"). Each variant is represented by a [`TwVariant`], which must be an enum with a default case.

The classes are merged in the following order, with the last class takes precedence:
1. Base class from [`TwClass`]
2. Base class from [`TwVariant`]
3. Enum variant class from [`TwVariant`]
4. Override class using [`IntoTailwindClass::with_class`] on the struct or builder

```rust
use tailwind_fuse::*;

// Your Component Type
#[derive(TwClass)]
// Optional base class
#[tw(class = "flex")]
struct Btn {
    size: BtnSize,
    color: BtnColor,
}

// Variant for size
#[derive(TwVariant)]
enum BtnSize {
    #[tw(default, class = "h-9 px-4 py-2")]
    Default,
    #[tw(class = "h-8 px-3")]
    Sm,
    #[tw(class = "h-10 px-8")]
    Lg,
}

// Variant for color
#[derive(TwVariant)]
enum BtnColor {
    #[tw(default, class = "bg-blue-500 text-blue-100")]
    Blue,
    #[tw(class = "bg-red-500 text-red-100")]
    Red,
}
```

You can now use the `Btn` struct to generate Tailwind classes, using builder syntax, or using the struct directly

### Struct Syntax
```rust
let button = Btn {
    size: BtnSize::Default,
    color: BtnColor::Blue,
};

assert_eq!(
   "flex h-9 px-4 py-2 bg-blue-500 text-blue-100",
   button.to_class()
);

// Conflicts are resolved (bg-blue-500 is knocked out in favor of override)
assert_eq!(
   "flex h-9 px-4 py-2 text-blue-100 bg-green-500",
   button.with_class("bg-green-500")
);
```

### Builder Syntax
You access the builder using the `variants` method. Every variant that is not provided will be replaced with the default variant.

```rust

assert_eq!(
   "flex h-8 px-3 bg-red-500 text-red-100",
   Btn::builder()
      .size(BtnSize::Sm)
      .color(BtnColor::Red)
      .to_class()
);

assert_eq!(
   "flex h-8 px-3 text-red-100 bg-green-500",
   Btn::builder()
      .size(BtnSize::Sm)
      .color(BtnColor::Red)
      .with_class("bg-green-500")
);

```

#### VSCode Intellisense

You can enable autocompletion inside `#[tw()]` using the steps below:

1. [Install the "Tailwind CSS IntelliSense" Visual Studio Code extension](https://marketplace.visualstudio.com/items?itemName=bradlc.vscode-tailwindcss)

2. Add the following to your [`settings.json`](https://code.visualstudio.com/docs/getstarted/settings):

```json
{
  "tailwindCSS.experimental.classRegex": [
    ["#[tw\\\\([^\\]]*class\\s*=\\s*\"([^\"]*)\"\\)]", "\"([^\"]*)\""]
  ]
}
```

<!-- cargo-rdme end -->
