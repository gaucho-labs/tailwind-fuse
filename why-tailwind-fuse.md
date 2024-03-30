# Why Tailwind Fuse?

When working with Tailwind CSS, it's common to apply multiple utility classes to an element to achieve the desired styling. However, as the number of classes grows, or when building reusable components, resolving conflicts can become a challenge. 

Tailwind Fuse is the missing piece for seamless class composition and conflict resolution when using Tailwind CSS with Rust.

In traditional CSS, conflicts between styles are resolved based on the order in which they are defined in the stylesheet, NOT the order in the html

### styles.css
```css
.text-red {
  color: red;
}

.text-blue {
  color: blue;
}
```

### index.html
```html
<div class="text-blue text-red">Hello, world!</div>
```

In this case, the text color of the `<div>` element will be blue, even though both `text-red` and `text-blue` classes are applied.

With Tailwind, it's common to apply multiple utility classes to an element, and the order of those classes in the HTML can affect the final styling.

This is where Tailwind Fuse shines. It introduces the Fuse utility, which intelligently merges multiple Tailwind classes while automatically resolving conflicts. 

Here's an example of how Tailwind Fuse handles conflicts:

```rust
use tailwind_fuse::*;

// Rightmost class takes precedence
assert_eq!("p-4", tw_merge!("p-2", "p-4"));

// Refinements are permitted!
assert_eq!("p-4 py-2", tw_merge!("p-4", "py-2"));

// Collisions can be complex
assert_eq!(
    "stroke-black stroke-1",
    tw_merge!("stroke-black", "stroke-1")
);

// Classes are unique per prefix
assert_eq!(
  "flex-col lg:flex-row",
  tw_merge!("flex-col", "lg:flex-row")
)

```

## We've also seen this before 🤮

```rust
use leptos::*;

#[component]
fn SomeComponent(#[prop(into, optional)] class: String) -> impl IntoView {
    view!{
        <div class={format!("flex text-red {class}")}>
          super steezy
        </div>
    }
}
```
