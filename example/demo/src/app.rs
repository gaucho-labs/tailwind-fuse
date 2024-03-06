use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_hotkeys::prelude::*;

use crate::{
    button::*,
    title::*,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let h_ref = provide_hotkeys_context(false, scopes!("home"));

    use_hotkeys!(("ctrl+r") => move |_| {
        window().location().set_href("https://github.com/gaucho-labs").expect("Failed to navigate");
    });

    view! {
        <Stylesheet id="leptos" href="/pkg/demo.css"/>
        <main _ref=h_ref>
            <Router>
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/:else" view=ErrorPage/>
                </Routes>
            </Router>
        </main>
    }
}

#[component]
fn HomePage() -> impl IntoView {

    let toggle_default = create_rw_signal(false);

    use_hotkeys!(("s", "home") => move |_| {
        if toggle_default.get() {
            toggle_default.set(false);
        } else {
            toggle_default.set(true);
        }
    });

    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    let buttons = vec![
        ("Conditionally display Button components using <Show />",
            view! {
                 <Show
                        when=move || toggle_default.get()
                        fallback=|| {
                            view! { <Button size=BtnSize::Lg>Press S to change size</Button> }
                        }
                    >
                        <Button size=BtnSize::Sm>Press S to change size</Button>
                    </Show>
            }
        ),
        ("Click on the button to increase the count",
            view! {
                 <Button on:click=on_click size=BtnSize::Sm variant=BtnVariant::Secondary>
                "Click Me: "
                {count}
            </Button>
            }
        )
    ];

    view! {
        <div class="h-screen bg-white p-8">
            <div class="flex space-x-2">
                <Button
                    on:click=move |_| {
                        window()
                            .location()
                            .set_href("https://github.com/gaucho-labs")
                            .expect("Failed to navigate")
                    }
                    variant=BtnVariant::Naked
                >
                    gaucho-labs
                </Button>
                <Button
                    on:click=move |_| {
                        window()
                            .location()
                            .set_href("https://github.com/gaucho-labs/tailwind-fuse")
                            .expect("Failed to navigate")
                    }
                    variant=BtnVariant::Naked
                >
                    tailwind-fuse
                </Button>
            </div>
            <div class="flex flex-col divide-y mt-8">
                <p class="text-3xl font-semibold p-4">Buttons</p>
                {
                    let b = buttons
                        .into_iter()
                        .map(|(title, view)| {
                            view! {
                                <div class="grid grid-cols-4 w-full divide-x">
                                    <p class="p-4">{title}</p>
                                    <div class="p-4 col-span-3">{view}</div>
                                </div>
                            }
                        })
                        .collect::<Vec<_>>();
                    b
                }

            </div>
        </div>
    }
}

#[component]
fn ErrorPage() -> impl IntoView {
    let params = use_params_map();
    let p_unknown = move || params.with(|p| p.get("else").cloned().unwrap_or_default());

    let unknown = p_unknown();

    view! {
        <main class=format!(
            "h-screen w-full flex flex-col items-center justify-center font-robotomono",
        )>
            <p class="">Unknown command: {unknown}</p>
        </main>
    }
}
