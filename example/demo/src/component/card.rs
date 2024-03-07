use leptos::*;
use tailwind_fuse::*;

#[component]
pub fn Card(
    #[prop(into, optional)] class: Option<MaybeSignal<String>>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let class = move || {
        tw_merge!(
            "rounded-xl border bg-card text-card-foreground shadow",
            class()
        )
    };

    view! {
        <div {..attributes} class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CardHeader(
    #[prop(into, optional)] class: Option<MaybeSignal<String>>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let class = move || tw_merge!("flex flex-col space-y-1.5 p-6", class());

    view! {
        <div {..attributes} class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CardTitle(
    #[prop(into, optional)] class: Option<MaybeSignal<String>>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let class = move || tw_merge!("font-semibold leading-none tracking-tight", class());

    view! {
        <h3 {..attributes} class=class>
            {children()}
        </h3>
    }
}

#[component]
pub fn CardDescription(
    #[prop(into, optional)] class: Option<MaybeSignal<String>>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let class = move || tw_merge!("text-sm text-muted-foreground", class());

    view! {
        <p {..attributes} class=class>
            {children()}
        </p>
    }
}

#[component]
pub fn CardContent(
    #[prop(into, optional)] class: Option<MaybeSignal<String>>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let class = move || tw_merge!("p-6 pt-0", class());

    view! {
        <div {..attributes} class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CardFooter(
    #[prop(into, optional)] class: Option<MaybeSignal<String>>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let class = move || tw_merge!("flex items-center p-6 pt-0", class());

    view! {
        <div {..attributes} class=class>
            {children()}
        </div>
    }
}
