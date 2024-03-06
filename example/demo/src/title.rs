use leptos::*;
use tailwind_fuse::*;


#[component]
pub fn Text(
    #[prop(into, optional)] variant: Option<TextVariant>,
    #[prop(into, optional)] class: Option<String>,
    #[prop(attrs)]
    attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {

    let text = Text {
        variant: variant.unwrap_or_default(),
    };

    view! {
        <p {..attributes} class=class>
            {children()}
        </p>
    }
}



#[derive(TwClass, Default)]
#[tw(
class = "font-roboto"
)]
pub struct Text {
    variant: TextVariant,
}

#[derive(TwVariant)]
pub enum TextVariant {
    #[tw(
    default,
    class = "text-lg"
    )]
    Default,

    #[tw(
    class = "text-2xl"
    )]
    Secondary,
}


