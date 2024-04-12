use leptos::*;
use tailwind_fuse::*;

#[component]
pub fn Button(
    #[prop(into, optional)] variant: MaybeSignal<BtnVariant>,
    #[prop(into, optional)] size: MaybeSignal<BtnSize>,
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        let variant = variant.get();
        let size = size.get();
        let button = Btn { variant, size };
        button.with_class(class.get())
    });

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
        class = "bg-white text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50"
    )]
    Secondary,
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
