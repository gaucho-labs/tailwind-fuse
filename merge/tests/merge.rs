use tw_merge::*;

fn tw_merge(class: &str) -> String {
    tw_merge::merge::tw_merge(class).unwrap()
}

#[test]
fn test_tw_merge() {
    let classes = tw_merge!("bg-red-500", "bg-blue-500", "text-green-500");
    assert_eq!(classes, "bg-blue-500 text-green-500")
}

#[test]
fn test_conflict() {
    let class = "bg-red-500 bg-blue-500 text-green-500";
    let result = tw_merge(class);
    assert_eq!(result, "bg-blue-500 text-green-500");

    let class = "bg-red-500 bg-blue-500 text-green-500 bg-amber-500";
    let result = tw_merge(class);
    assert_eq!(result, "text-green-500 bg-amber-500");

    let class = "bg-orange-300 bg-[#f5f5f5] text-green-400 text-orange-300";
    let result = tw_merge(class);
    assert_eq!(result, "bg-[#f5f5f5] text-orange-300");
}

#[test]
fn test_conflict_with_modifiers() {
    let class = "bg-red-500 bg-blue-500 hover:bg-green-500 text-green-500";
    let result = tw_merge(class);
    assert_eq!(result, "bg-blue-500 hover:bg-green-500 text-green-500");

    let class = "bg-red-500 bg-blue-500 hover:bg-green-500 text-green-500 hover:text-red-500";
    let result = tw_merge(class);
    assert_eq!(
        result,
        "bg-blue-500 hover:bg-green-500 text-green-500 hover:text-red-500"
    );

    let class = "hover:scale-1.5 hover:bg-orange-300";
    let result = tw_merge(class);
    assert_eq!(&result, class)
}

#[test]
fn test_conflict_with_arbitrary_values() {
    let class = "m-[2px] m-[10px]";
    let result = tw_merge(class);
    assert_eq!(result, "m-[10px]");

    let class = "m-[2px] m-[11svmin] m-[12in] m-[13lvi] m-[14vb] m-[15vmax] m-[16mm] m-[17%] m-[18em] m-[19px] m-[10dvh]";
    let result = tw_merge(class);
    assert_eq!(result, "m-[10dvh]");

    let class = "h-[10px] h-[11cqw] h-[12cqh] h-[13cqi] h-[14cqb] h-[15cqmin] h-[16cqmax]";
    let result = tw_merge(class);
    assert_eq!(result, "h-[16cqmax]");

    let class = "z-20 z-[99]";
    let result = tw_merge(class);
    assert_eq!(result, "z-[99]");

    let class = "my-[2px] m-[10rem]";
    let result = tw_merge(class);
    assert_eq!(result, "m-[10rem]");

    let class = "cursor-pointer cursor-[grab]";
    let result = tw_merge(class);
    assert_eq!(result, "cursor-[grab]");

    let class = "m-[2px] m-[calc(100%-var(--arbitrary))]";
    let result = tw_merge(class);
    assert_eq!("m-[calc(100%-var(--arbitrary))]", result);

    let class = "m-[2px] m-[length:var(--mystery-var)]";
    let result = tw_merge(class);
    assert_eq!(result, "m-[length:var(--mystery-var)]");

    let class = "opacity-10 opacity-[0.025]";
    let result = tw_merge(class);
    assert_eq!(result, "opacity-[0.025]");

    let class = "scale-75 scale-[1.7]";
    let result = tw_merge(class);
    assert_eq!("scale-[1.7]", result);

    let class = "brightness-90 brightness-[1.75]";
    let result = tw_merge(class);
    assert_eq!("brightness-[1.75]", result);

    // Handling of value `0`
    let class = "min-h-[0.5px] min-h-[0]";
    let result = tw_merge(class);
    assert_eq!("min-h-[0]", result);

    let class = "text-[0.5px] text-[color:0]";
    let result = tw_merge(class);
    assert_eq!(result, "text-[0.5px] text-[color:0]");

    let class = "text-[0.5px] text-[--my-0]";
    let result = tw_merge(class);
    assert_eq!(result, "text-[0.5px] text-[--my-0]");

    let class = "hover:m-[2px] hover:m-[length:var(--c)]";
    let result = tw_merge(class);
    assert_eq!(result, "hover:m-[length:var(--c)]");

    let class = "focus:hover:m-[2px] focus:hover:m-[length:var(--c)]";
    let result = tw_merge(class);
    assert_eq!(result, "focus:hover:m-[length:var(--c)]");

    let class = "border-b border-[color:rgb(var(--color-gray-500-rgb)/50%))]";
    let result = tw_merge(class);
    assert_eq!(
        result,
        "border-b border-[color:rgb(var(--color-gray-500-rgb)/50%))]"
    );

    let class = "border-[color:rgb(var(--color-gray-500-rgb)/50%))] border-b";
    let result = tw_merge(class);
    assert_eq!(
        result,
        "border-[color:rgb(var(--color-gray-500-rgb)/50%))] border-b"
    );

    let class = "border-b border-[color:rgb(var(--color-gray-500-rgb)/50%))] border-some-coloooor";
    let result = tw_merge(class);
    assert_eq!(result, "border-b border-some-coloooor");

    let class = "grid-rows-[1fr,auto] grid-rows-2";
    let result = tw_merge(class);
    assert_eq!(result, "grid-rows-2");

    let class = "grid-rows-[repeat(20,minmax(0,1fr))] grid-rows-3";
    let result = tw_merge(class);
    assert_eq!(result, "grid-rows-3");

    let class = "mt-2 mt-[calc(theme(fontSize.4xl)/1.125)]";
    let result = tw_merge(class);
    assert_eq!(result, "mt-[calc(theme(fontSize.4xl)/1.125)]");

    let class = "p-2 p-[calc(theme(fontSize.4xl)/1.125)_10px]";
    let result = tw_merge(class);
    assert_eq!(result, "p-[calc(theme(fontSize.4xl)/1.125)_10px]");

    let class = "mt-2; mt-[length:theme(someScale.someValue)]";
    let result = tw_merge(class);
    assert_eq!(result, "mt-[length:theme(someScale.someValue)]");

    let class = "mt-2 mt-[theme(someScale.someValue)]";
    let result = tw_merge(class);
    assert_eq!(result, "mt-[theme(someScale.someValue)]");

    let class = "text-2xl text-[length:theme(someScale.someValue)]";
    let result = tw_merge(class);
    assert_eq!(result, "text-[length:theme(someScale.someValue)]");

    let class = "text-2xl text-[calc(theme(fontSize.4xl)/1.125)]";
    let result = tw_merge(class);
    assert_eq!(result, "text-[calc(theme(fontSize.4xl)/1.125)]");

    let class = "bg-cover bg-[percentage:30%] bg-[length:200px_100px]";
    let result = tw_merge(class);
    assert_eq!(result, "bg-[length:200px_100px]");

    let class =
        "bg-none bg-[url(.)] bg-[image:.] bg-[url:.] bg-[linear-gradient(.)] bg-gradient-to-r";
    let result = tw_merge(class);
    assert_eq!(result, "bg-gradient-to-r");
}

#[test]
fn test_merge_with_non_tailwind() {
    let class_names: String = vec![
        "header",
        "nav-bar",
        "feature",
        // tailwind class
        "flex-row",
        "section",
        "container-fluid",
        "row",
        "column",
        // tailwind class
        "flex-col",
    ]
    .into_iter()
    .map(String::from)
    .collect::<Vec<_>>()
    .join(" ");

    let result = tw_merge(&class_names);

    // flex row should be removed in favor of flex-col.
    let expected = class_names.replace(" flex-row", "");

    assert_eq!(result, expected);
}

#[test]
fn handles_arbitrary_length_conflicts_with_labels_and_modifiers_correctly() {
    assert_eq!(
        tw_merge("hover:m-[2px] hover:m-[length:var(--c)]"),
        "hover:m-[length:var(--c)]"
    );
    assert_eq!(
        tw_merge("hover:focus:m-[2px] hover:focus:m-[length:var(--c)]"),
        "hover:focus:m-[length:var(--c)]"
    );
    assert_eq!(
        tw_merge("border-b border-[color:rgb(var(--color-gray-500-rgb)/50%))]"),
        "border-b border-[color:rgb(var(--color-gray-500-rgb)/50%))]"
    );
    assert_eq!(
        tw_merge("border-[color:rgb(var(--color-gray-500-rgb)/50%))] border-b"),
        "border-[color:rgb(var(--color-gray-500-rgb)/50%))] border-b"
    );
    assert_eq!(
        tw_merge(
            "border-b border-[color:rgb(var(--color-gray-500-rgb)/50%))] border-some-coloooor"
        ),
        "border-b border-some-coloooor"
    );
}

#[test]
fn handles_complex_arbitrary_value_conflicts_correctly() {
    assert_eq!(tw_merge("grid-rows-[1fr,auto] grid-rows-2"), "grid-rows-2");
    assert_eq!(
        tw_merge("grid-rows-[repeat(20,minmax(0,1fr))] grid-rows-3"),
        "grid-rows-3"
    );
}

#[test]
fn handles_ambiguous_arbitrary_values_correctly() {
    assert_eq!(
        tw_merge("mt-2 mt-[calc(theme(fontSize.4xl)/1.125)]"),
        "mt-[calc(theme(fontSize.4xl)/1.125)]"
    );
    assert_eq!(
        tw_merge("p-2 p-[calc(theme(fontSize.4xl)/1.125)_10px]"),
        "p-[calc(theme(fontSize.4xl)/1.125)_10px]"
    );
    assert_eq!(
        tw_merge("mt-2 mt-[length:theme(someScale.someValue)]"),
        "mt-[length:theme(someScale.someValue)]"
    );
    assert_eq!(
        tw_merge("mt-2 mt-[theme(someScale.someValue)]"),
        "mt-[theme(someScale.someValue)]"
    );

    assert_eq!(
        tw_merge("text-2xl text-[length:theme(someScale.someValue)]"),
        "text-[length:theme(someScale.someValue)]"
    );
    assert_eq!(
        tw_merge("text-2xl text-[calc(theme(fontSize.4xl)/1.125)]"),
        "text-[calc(theme(fontSize.4xl)/1.125)]"
    );
    assert_eq!(
        tw_merge("bg-cover bg-[percentage:30%] bg-[length:200px_100px]"),
        "bg-[length:200px_100px]"
    );
    assert_eq!(
        tw_merge(
            "bg-none bg-[url(.)] bg-[image:.] bg-[url:.] bg-[linear-gradient(.)] bg-gradient-to-r"
        ),
        "bg-gradient-to-r"
    );
}

#[test]
fn handles_color_conflicts_properly() {
    assert_eq!(tw_merge("bg-grey-5 bg-hotpink"), "bg-hotpink");
    assert_eq!(
        tw_merge("hover:bg-grey-5 hover:bg-hotpink"),
        "hover:bg-hotpink"
    );
    assert_eq!(
        tw_merge("stroke-[hsl(350_80%_0%)] stroke-[10px]"),
        "stroke-[hsl(350_80%_0%)] stroke-[10px]"
    );
}

#[test]
fn merges_content_utilities_correctly() {
    assert_eq!(
        tw_merge("content-['hello'] content-[attr(data-content)]"),
        "content-[attr(data-content)]"
    );
}

#[test]
fn merges_tailwind_classes_with_important_modifier_correctly() {
    assert_eq!(tw_merge("!font-medium !font-bold"), "!font-bold");
    assert_eq!(
        tw_merge("!font-medium !font-bold font-thin"),
        "!font-bold font-thin"
    );
    assert_eq!(tw_merge("!right-2 !-inset-x-px"), "!-inset-x-px");
    assert_eq!(tw_merge("focus:!inline focus:!block"), "focus:!block");
}

#[test]
fn merges_classes_with_per_side_border_colors_correctly() {
    assert_eq!(
        tw_merge("border-t-some-blue border-t-other-blue"),
        "border-t-other-blue"
    );
    assert_eq!(
        tw_merge("border-t-some-blue border-some-blue"),
        "border-some-blue"
    );

    assert_eq!(
        tw_merge("border-t-some-blue border-y-some-blue"),
        "border-y-some-blue"
    );

    assert_eq!(
        tw_merge("border-l-some-blue border-x-some-blue"),
        "border-x-some-blue"
    );
}

#[test]
fn test_data_attributes() {
    assert_eq!(
        tw_merge("data-[open]:flex-col data-[close]:flex-row"),
        "data-[open]:flex-col data-[close]:flex-row"
    );
}

#[test]
fn basic_arbitrary_variants() {
    assert_eq!(
        tw_merge("[&>*]:underline [&>*]:line-through"),
        "[&>*]:line-through"
    );
    assert_eq!(
        tw_merge("[&>*]:underline [&>*]:line-through [&_div]:line-through"),
        "[&>*]:line-through [&_div]:line-through"
    );
    // TODO: FIX
    // assert_eq!(
    //     tw_merge("supports-[display:grid]:flex supports-[display:grid]:grid"),
    //     "supports-[display:grid]:grid"
    // );
}

#[test]
fn arbitrary_variants_with_modifiers() {
    assert_eq!(
        tw_merge("dark:lg:hover:[&>*]:underline dark:lg:hover:[&>*]:line-through"),
        "dark:lg:hover:[&>*]:line-through"
    );
    assert_eq!(
        tw_merge("hover:[&>*]:underline [&>*]:hover:line-through"),
        "hover:[&>*]:underline [&>*]:hover:line-through"
    );
    assert_eq!(
        tw_merge(
            "dark:hover:[&>*]:underline dark:hover:[&>*]:underline dark:[&>*]:hover:line-through"
        ),
        "dark:hover:[&>*]:underline dark:[&>*]:hover:line-through"
    );
}

#[test]
fn arbitrary_variants_with_complex_syntax_in_them() {
    assert_eq!(tw_merge("[@media_screen{@media(hover:hover)}]:underline [@media_screen{@media(hover:hover)}]:line-through"), "[@media_screen{@media(hover:hover)}]:line-through");
    assert_eq!(tw_merge("hover:[@media_screen{@media(hover:hover)}]:underline hover:[@media_screen{@media(hover:hover)}]:line-through"), "hover:[@media_screen{@media(hover:hover)}]:line-through");
}

#[test]
fn arbitrary_variants_with_attribute_selectors() {
    assert_eq!(
        tw_merge("[&[data-open]]:underline [&[data-open]]:line-through"),
        "[&[data-open]]:line-through"
    );
}

#[test]
fn arbitrary_variants_with_multiple_attribute_selectors() {
    assert_eq!(tw_merge("[&[data-foo][data-bar]:not([data-baz])]:underline [&[data-foo][data-bar]:not([data-baz])]:line-through"), "[&[data-foo][data-bar]:not([data-baz])]:line-through");
}

#[test]
fn multiple_arbitrary_variants() {
    assert_eq!(
        tw_merge("[&>*]:[&_div]:underline [&>*]:[&_div]:line-through"),
        "[&>*]:[&_div]:line-through"
    );
    assert_eq!(
        tw_merge("[&>*]:[&_div]:underline [&_div]:[&>*]:line-through"),
        "[&>*]:[&_div]:underline [&_div]:[&>*]:line-through"
    );
    // Does order matter?
    // assert_eq!(tw_merge("hover:dark:[&>*]:focus:disabled:[&_div]:underline dark:hover:[&>*]:disabled:focus:[&_div]:line-through"), "dark:hover:[&>*]:disabled:focus:[&_div]:line-through");
    assert_eq!(tw_merge("hover:dark:[&>*]:focus:[&_div]:disabled:underline dark:hover:[&>*]:disabled:focus:[&_div]:line-through"), "hover:dark:[&>*]:focus:[&_div]:disabled:underline dark:hover:[&>*]:disabled:focus:[&_div]:line-through");
}

// TODO: Fix this maybe?
// #[test]
// fn arbitrary_variants_with_arbitrary_properties() {
//     assert_eq!(
//         tw_merge("[&>*]:[color:red] [&>*]:[color:blue]"),
//         "[&>*]:[color:blue]"
//     );
//     assert_eq!(tw_merge("[&[data-foo][data-bar]:not([data-baz])]:nod:noa:[color:red] [&[data-foo][data-bar]:not([data-baz])]:noa:nod:[color:blue]"), "[&[data-foo][data-bar]:not([data-baz])]:noa:nod:[color:blue]");
// }
