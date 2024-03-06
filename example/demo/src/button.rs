use leptos::*;
use tailwind_fuse::*;

#[component]
pub fn Button(
    #[prop(into, optional)] size: Option<BtnSize>,
    #[prop(into, optional)] variant: Option<BtnVariant>,
    #[prop(into, optional)] class: Option<String>,
    /// Arbitrary additional attributes.
    #[prop(attrs)]
    attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let btn = Btn {
        size: size.unwrap_or_default(),
        variant: variant.unwrap_or_default(),
    };

    let class = class.map_or(btn.to_class(), |c| btn.with_class(c));

    view! {
        <button {..attributes} class=class>
            {children()}
        </button>
    }
}

#[derive(TwClass, Default)]
#[tw(
class = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors disabled:pointer-events-none disabled:opacity-50"
)]
pub struct Btn {
    variant: BtnVariant,
    size: BtnSize,
}

#[derive(TwVariant)]
pub enum BtnVariant {
    #[tw(
    default,
    class = "bg-indigo-600 text-white hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
    )]
    Default,
    #[tw(
    class = "bg-gray-100 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-200"
    )]
    Secondary,

    #[tw(
    class= "text-gray-900 hover:underline"
    )]
    Naked,
}

#[derive(TwVariant)]
pub enum BtnSize {
    #[tw(class = "h-8 px-3 text-xs")]
    Sm,
    #[tw(default, class = "h-9 px-4 py-2")]
    Default,
    #[tw(class = "h-10 px-8")]
    Lg,
}
