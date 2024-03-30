use leptos::*;
use tailwind_fuse::*;

#[component]
fn H1(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let class = tw_merge!(
        "scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl",
        class
    );
    view! {
        <h1 class={class}>
            {children()}
        </h1>
    }
}

#[component]
fn H2(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let class = tw_merge!(
        "scroll-m-20 border-b pb-2 text-xl font-semibold tracking-tight",
        class
    );
    view! {
        <h2 class={class}>
            {children()}
        </h2>
    }
}

#[component]
fn H4(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let class = tw_merge!("scroll-m-20 text-xl font-semibold tracking-tight", class);
    view! {
        <h4 class={class}>
            {children()}
        </h4>
    }
}
