use crate::{
    button::{BtnSize, BtnVariant, Button},
    error_template::{AppError, ErrorTemplate},
};
use leptos::{logging::log, *};
use leptos_meta::*;
use leptos_router::*;
use tailwind_fuse::{merge::MergeOptions, *};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    tailwind_fuse::merge::set_tw_options(MergeOptions {
        prefix: "tw-",
        separator: ":",
    });

    view! {
        <Stylesheet id="leptos" href="/pkg/start-axum.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    let options = MergeOptions::default();
    log!("MergeOptions {options:?}");

    view! {
        // flex-row should be removed.
        <div class=tw_merge!("tw-flex tw-items-center tw-gap-4 tw-p-10 tw-flex-row", "tw-flex-col")>
            <Button on:click=on_click size=BtnSize::Lg>
                "Click Me: "
                {count}
            </Button>
            <Button on:click=on_click size=BtnSize::Sm variant=BtnVariant::Secondary>
                "Click Me: "
                {count}
            </Button>
        </div>
    }
}
