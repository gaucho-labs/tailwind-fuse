use leptos::*;
use tailwind_fuse::*;

#[component]
pub fn Button(
    #[prop(into, optional)] variant: Option<MaybeSignal<ButtonVariant>>,
    #[prop(into, optional)] size: Option<MaybeSignal<ButtonSize>>,
    #[prop(into, optional)] class: Option<MaybeSignal<String>>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_default();

    let class = Signal::derive(move || {
        let variant = variant.unwrap_or_default()();
        let size = size.unwrap_or_default()();
        let button = ButtonClass { variant, size };
        button.with_class(class())
    });

    view! {
        <button {..attributes} class=class>
            {children()}
        </button>
    }
}

#[derive(TwClass, Clone, Copy)]
#[tw(
    class = r#"inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors 
               focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50"#
)]
pub struct ButtonClass {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
}

#[derive(TwVariant, Clone, Copy)]
pub enum ButtonVariant {
    #[tw(
        default,
        class = "bg-primary text-primary-foreground shadow hover:bg-primary/90"
    )]
    Default,
    #[tw(class = "bg-destructive text-destructive-foreground shadow-sm hover:bg-destructive/90")]
    Destructive,
    #[tw(
        class = "border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground"
    )]
    Outline,
    #[tw(class = "bg-secondary text-secondary-foreground shadow-sm hover:bg-secondary/80")]
    Secondary,
    #[tw(class = "hover:bg-accent hover:text-accent-foreground")]
    Ghost,
    #[tw(class = "text-primary underline-offset-4 hover:underline")]
    Link,
}

#[derive(TwVariant, Clone, Copy)]
pub enum ButtonSize {
    #[tw(default, class = "h-9 px-4 py-2")]
    Default,
    #[tw(class = "h-8 rounded-md px-3 text-xs")]
    Sm,
    #[tw(class = "h-10 rounded-md px-8")]
    Lg,
    #[tw(class = "h-9 w-9")]
    Icon,
}
