# Tailwind Fuse 

Two main utils are included in this crate:

1. Tailwind Fuse: Fuse multiple tailwind classes, with optional conflict resolution.
    > Inspired by [Tailwind Merge](https://github.com/dcastil/tailwind-merge)
2. Tailwind Variants: Compose type-safe variant classes
    > Inspired by [Class Variance Authority](https://github.com/joe-bell/cva)


## Installation

Variants requires the `variants` feature to be enabled.

#### With variants
```bash
cargo add tailwind-fuse --features variants
```

#### Without variants
```bash
cargo add tailwind-fuse
```

## Usage

### Tailwind Fuse

You can use `tw_join!` to join tailwind classes, and `tw_merge!` to merge tailwind classes handling conflicts.

```rust
use tailwind_fuse::*;

// No conflict resolution
// "flex items-center justify-center"
let joined_class = tw_join!("flex items-center", "justify-center");

// You can use Option to handle conditional rendering
// You can pass in &str, String, Option<String>, or Option<&str>
// "text-sm font-bold"
let classes = tw_join!(
    "text-sm",
    Some("font-bold"),
    None::<String>,
    Some("ring").filter(|_| false),
    Some(" "),
    "".to_string(),
);


// Conflict resolution
// Right most class takes precedence
// p-4
let merged_class = tw_merge!("py-2 px-4", "p-4");

// Refinements are permitted
// p-4 py-2
let merged_class = tw_merge!("p-4", "py-2");

```

### Tailwind Variants

Useful for building components with first class support for tailwind. By default, conflicts are merged using `tw_merge`.

Each `TwClass` represents a UI element with customizable properties (property is a "variant"). Each variant is represented by a `TwVariant`, which must be an enum with a default case.

The merge order is, where the last class takes preferences:
1. TwClass base class
2. TwVariant base class
3. TwVariant enum variant class
4. Override class `with_class`

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

You can now use the `Btn` struct to generate tailwind classes, using builder syntax, or using the struct directly


#### Struct Syntax
```rust
let button = Btn {
    size: BtnSize::Default,
    color: BtnColor::Blue,
};
// h-9 px-4 py-2 bg-blue-500 text-blue-100
button.to_class();
// Conflicts are resolved.
// h-9 px-4 py-2 text-blue-100 bg-green-500
button.with_class("bg-green-500");

```

#### Builder Syntax
You access the builder using the `variants` method. Every variant that is not provided will be replaced with the default variant.

```rust

// h-8 px-3 bg-red-500 text-red-100
let class = Btn::variant()
    .size(BtnSize::Sm)
    .color(BtnColor::Red)
    .to_class();

// h-8 px-3 text-red-100 bg-green-500
let class = Btn::variant()
    .size(BtnSize::Sm)
    .color(BtnColor::Red)
    .with_class("bg-green-500");

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