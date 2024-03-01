use tw_merge::merge::tw_merge;
use tw_merge::*;

#[test]
fn test_tw_merge_mixed_blend() {
    let classes = tw_merge!("mix-blend-normal", "mix-blend-multiply");
    assert_eq!(classes, "mix-blend-multiply");
}

#[test]
fn test_tw_merge_height() {
    let classes = tw_merge!("h-10", "h-min");
    assert_eq!(classes, "h-min");
}

#[test]
fn test_tw_merge_stroke() {
    let classes = tw_merge!("stroke-black", "stroke-1");
    assert_eq!(classes, "stroke-black stroke-1");

    let classes = tw_merge!("stroke-2", "stroke-[3px]");
    assert_eq!(classes, "stroke-[3px]");

    let classes = tw_merge!("stroke-black", "stroke-red-500", "stroke-blue-100");
    assert_eq!(classes, "stroke-blue-100");
}

#[test]
fn test_tw_merge_outline() {
    let classes = tw_merge!("outline-black", "outline-1");
    assert_eq!(classes, "outline-black outline-1");
}

#[test]
fn test_tw_merge_grayscale() {
    let classes = tw_merge!("grayscale-0", "grayscale-[50%]");
    assert_eq!(classes, "grayscale-[50%]");
}

#[test]
fn test_padding_narrowing() {
    let classes = tw_merge!("p-10", "px-5");
    assert_eq!(classes, "p-10 px-5");
    let classes = tw_merge!("px-5", "py-5", "p-10",);
    assert_eq!(classes, "p-10");
}

#[test]
fn test_gap_narrowing() {
    let classes = tw_merge!("gap-10", "gap-x-5");
    assert_eq!(classes, "gap-10 gap-x-5");

    let classes = tw_merge!("gap-x-5", "gap-y-5", "gap-10");
    assert_eq!(classes, "gap-10");
}

#[test]
fn merges_classes_from_same_group_correctly() {
    let class = "overflow-x-auto overflow-x-hidden";
    let result = tw_merge(class).unwrap();
    assert_eq!(result, "overflow-x-hidden");

    let class = "basis-full basis-auto";
    let result = tw_merge(class).unwrap();
    assert_eq!(result, "basis-auto");

    let class = "w-full w-fit";
    let result = tw_merge(class).unwrap();
    assert_eq!(result, "w-fit");

    let class = "overflow-x-auto overflow-x-hidden overflow-x-scroll";
    let result = tw_merge(class).unwrap();
    assert_eq!(result, "overflow-x-scroll");

    let class = "overflow-x-auto hover:overflow-x-hidden overflow-x-scroll";
    let result = tw_merge(class).unwrap();
    assert_eq!(result, "hover:overflow-x-hidden overflow-x-scroll");

    let class = "overflow-x-auto hover:overflow-x-hidden hover:overflow-x-auto overflow-x-scroll";
    let result = tw_merge(class).unwrap();
    assert_eq!(result, "hover:overflow-x-auto overflow-x-scroll");

    let class = "col-span-1 col-span-full";
    let result = tw_merge(class).unwrap();
    assert_eq!(result, "col-span-full");
}

#[test]
fn merges_classes_from_font_variant_numeric_section_correctly() {
    let class = "lining-nums tabular-nums diagonal-fractions";
    let result = tw_merge(class).unwrap();
    assert_eq!(result, "lining-nums tabular-nums diagonal-fractions");

    let class = "normal-nums tabular-nums diagonal-fractions";
    let result = tw_merge(class).unwrap();
    assert_eq!(result, "tabular-nums diagonal-fractions");

    let class = "tabular-nums diagonal-fractions normal-nums";
    let result = tw_merge(class).unwrap();
    assert_eq!(result, "normal-nums");

    let class = "tabular-nums proportional-nums";
    let result = tw_merge(class).unwrap();
    assert_eq!(result, "proportional-nums");
}

#[test]
fn handles_color_conflicts_properly() {
    let class = "bg-grey-5 bg-hotpink";
    let result = tw_merge(class).unwrap();
    assert_eq!(result, "bg-hotpink");

    let class = "hover:bg-grey-5 hover:bg-hotpink";
    let result = tw_merge(class).unwrap();
    assert_eq!(result, "hover:bg-hotpink");

    let class = "stroke-[hsl(350_80%_0%)] stroke-[10px]";
    let result = tw_merge(class).unwrap();
    assert_eq!(result, "stroke-[hsl(350_80%_0%)] stroke-[10px]");
}
