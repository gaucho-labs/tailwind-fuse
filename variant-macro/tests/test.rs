use tw_utils::ToTailwindClass;
use tw_variant_macro::{TailwindClass, TailwindVariant};

#[derive(TailwindClass)]
#[class = "flex items-center"]
struct Btn {
    size: BtnSize,
    color: BtnColor,
}

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

#[derive(TailwindVariant)]
enum BtnColor {
    #[default]
    #[class("bg-blue-500 text-white")]
    Default,
    #[class("bg-red-500 text-white")]
    Red,
}

#[test]
fn test_btn_size() {
    assert_eq!(BtnSize::default().to_class(), "h-9 px-4 py-2");
    assert_eq!(BtnSize::Sm.to_class(), "h-8 rounded-md px-3 text-xs");
    assert_eq!(BtnSize::Lg.to_class(), "h-10 rounded-md px-8");
}

#[test]
fn test_btn() {
    let button = Btn {
        size: Default::default(),
        color: Default::default(),
    };
    assert_eq!(
        button.to_class(),
        "flex items-center h-9 px-4 py-2 bg-blue-500 text-white"
    )
}

#[test]
fn test_class_builder() {
    assert_eq!(
        Btn::variant()
            .size(BtnSize::Sm)
            .color(BtnColor::Red)
            .to_class(),
        "flex items-center h-8 rounded-md px-3 text-xs bg-red-500 text-white"
    );

    assert_eq!(
        Btn::variant()
            .size(BtnSize::Sm)
            .color(BtnColor::Red)
            .with_class("flex"),
        "flex items-center h-8 rounded-md px-3 text-xs bg-red-500 text-white flex"
    );

    assert_eq!(
        Btn::variant().to_class(),
        "flex items-center h-9 px-4 py-2 bg-blue-500 text-white"
    );

    assert_eq!(
        Btn::variant().with_class("grid"),
        "flex items-center h-9 px-4 py-2 bg-blue-500 text-white grid"
    );
}
