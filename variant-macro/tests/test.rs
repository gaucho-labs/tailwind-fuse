use variant::{TailwindClass, TailwindVariant};

#[derive(TailwindVariant)]
enum BtnSize {
    #[default]
    #[class("h-9 px-4 py-2")]
    Default,
    #[class("h-8 rounded-md px-3 text-xs")]
    Sm,
    #[class("h-10 rounded-md px-8")]
    Lg,
    #[class("h-9 w-9")]
    Icon,
}

#[derive(TailwindVariant)]
enum BtnColor {
    #[default]
    #[class("bg-blue-500 text-white")]
    Default,
    #[class("bg-red-500 text-white")]
    Red,
    #[class("bg-green-500 text-white")]
    Green,
}

#[derive(TailwindClass)]
struct Btn {
    size: BtnSize,
    color: BtnColor,
}

#[test]
fn test_default_btn_size() {
    let size = BtnSize::default();
    assert_eq!(size.to_class(), "h-9 px-4 py-2");
}

#[test]
fn test_class() {
    let btn = Btn {
        size: BtnSize::Sm,
        color: BtnColor::Red,
    };

    assert_eq!(
        btn.to_class(),
        "h-8 rounded-md px-3 text-xs bg-red-500 text-white"
    );

    assert_eq!(
        btn.with_class("flex"),
        "h-8 rounded-md px-3 text-xs bg-red-500 text-white flex"
    );
}
