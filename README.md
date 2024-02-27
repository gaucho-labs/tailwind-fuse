# Tailwind Utils (for Rust)

Two main utils are included in this crate:

1. Tailwind Merge: A utility to merge tailwind classes into a single class.
    > Inspired by [Tailwind Merge](https://github.com/dcastil/tailwind-merge)
2. Tailwind Variants: A utility to compose type-safe variant classes
    > Inspired by [Class Variance Authority](https://github.com/joe-bell/cva)


## Installation

Variants is feature gated and requires the `variants` feature to be enabled.

#### With variants
```bash
cargo add tailwind-utils --features variants
```

#### Without variants
```bash
cargo add tailwind-utils
```

## Usage

### Tailwind Merge

```rust
use tailwind_utils::*;

let class_a = "flex items-center";
let class_b = "justify-center";
// "flex items-center justify-center"
let joined_class = tailwind_utils::tw_join!(class_a, class_b);

let class_a = "py-2 px-4";
let class_b = "p-4"
// "p-4"
let merged_class = tailwind_utils::tw_merge!(class_a, class_b);
```

### Tailwind Variants

Each `TailwindVariant` must have a default variant, denoted by the `#[default]` attribute.

```rust
use tailwind_utils::*;

// Your Component Type
#[derive(TailwindClass)]
struct Btn {
    size: BtnSize,
    color: BtnColor,
}

// Variant for size
#[derive(TailwindVariant)]
enum BtnSize {
    #[default]
    #[class("h-9 px-4 py-2")]
    Default,
    #[class("h-8 rounded-md px-3 text-xs")]
    Sm,
    #[class("h-10 rounded-md px-8")]
    Lg,
}

// Variant for color
#[derive(TailwindVariant)]
enum BtnColor {
    #[default]
    #[class("bg-blue-500 text-white")]
    Default,
    #[class("bg-red-500 text-white")]
    Red,
}

```

You can now use the `Btn` struct to generate tailwind classes, using builder syntax, or using the struct directly

You access the builder using the `variants` method.

#### Struct Syntax
```rust
#[test]
fn test_btn() {
    let button = Btn {
        size: Default::default(),
        color: Default::default(),
    };
    assert_eq!(button.to_class(), "h-9 px-4 py-2 bg-blue-500 text-white")
}

```

#### Builder Syntax
Every variant that is not provided will be replaced with the default variant.

```rust
#[test]
fn test_class_builder() {
    assert_eq!(
        Btn::variant()
            .size(BtnSize::Sm)
            .color(BtnColor::Red)
            .to_class(),
        "h-8 rounded-md px-3 text-xs bg-red-500 text-white"
    );
}
```
