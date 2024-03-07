use leptos::*;
use leptos_hotkeys::prelude::*;
use leptos_meta::*;
use leptos_router::*;

use crate::component::badge::*;
use crate::component::button::*;
use crate::component::card::*;
use crate::title::*;

use tailwind_fuse::*;

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
        (
            "Conditionally display Button components using <Show />",
            view! {
                 <Show
                        when=move || toggle_default.get()
                        fallback=|| {
                            view! { <Button size=ButtonSize::Lg>Press S to change size</Button> }
                        }
                    >
                        <Button size=ButtonSize::Sm>Press S to change size</Button>
                    </Show>
            },
        ),
        (
            "Click on the button to increase the count",
            view! {
                 <Button on:click=on_click size=ButtonSize::Sm variant=ButtonVariant::Secondary>
                "Click Me: "
                {count}
            </Button>
            },
        ),
        (
            "Button sizes",
            view! {
                <div class="flex items-center gap-2">
                    <Button size=ButtonSize::Icon>Icon</Button>
                    <Button size=ButtonSize::Sm>Small</Button>
                    <Button>Default</Button>
                    <Button size=ButtonSize::Lg>Large</Button>
                </div>
            }
            .into_view(),
        ),
        (
            "Button variants",
            view! {
                <div class="flex items-center gap-2 flex-wrap">
                    <Button variant=ButtonVariant::Default>Default</Button>
                    <Button variant=ButtonVariant::Secondary>Secondary</Button>
                    <Button variant=ButtonVariant::Destructive>Destructive</Button>
                    <Button variant=ButtonVariant::Outline>Outline</Button>
                    <Button variant=ButtonVariant::Ghost>Ghost</Button>
                    <Button variant=ButtonVariant::Link>Link</Button>
                </div>
            }
            .into_view(),
        ),
        (
            "Badge variants",
            view! {
                <div class="flex items-center gap-2">
                    <Badge variant=BadgeVariant::Default>Default</Badge>
                    <Badge variant=BadgeVariant::Secondary>Secondary</Badge>
                    <Badge variant=BadgeVariant::Destructive>Destructive</Badge>
                    <Badge variant=BadgeVariant::Outline>Outline</Badge>
                </div>
            }
            .into_view(),
        ),
    ];

    view! {
        <div class="h-screen bg-white p-8">
            <div class="flex space-x-2">
                <a
                    class=ButtonClass::variant()
                        .size(ButtonSize::Sm)
                        .variant(ButtonVariant::Link)
                        .to_class()
                    href="https://github.com/gaucho-labs"
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    gaucho-labs
                </a>
                <a
                    class=ButtonClass::variant()
                        .size(ButtonSize::Sm)
                        .variant(ButtonVariant::Link)
                        .to_class()
                    href="https://github.com/gaucho-labs/tailwind-fuse"
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    tailwind-fuse
                </a>
            </div>
            <div class="flex flex-col items-center py-8 gap-4 w-full">
                <h4 class="text-3xl font-semibold p-4">Buttons</h4>
                {
                    buttons
                    .into_iter()
                    .map(|(title, view)| {
                        view! {
                            <Card class="max-w-lg lg:max-w-2xl w-full overflow-none">
                                <CardHeader>
                                    <CardTitle>{title}</CardTitle>
                                </CardHeader>
                                <CardContent>
                                    {view}
                                </CardContent>
                                <CardFooter>
                                    <div></div>
                                </CardFooter>
                            </Card>
                        }
                    })
                    .collect::<Vec<_>>()
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
        <main class=tw_join!(
            "h-screen w-full flex flex-col items-center justify-center font-robotomono",
        )>
            <p class="">Unknown command: {unknown}</p>
        </main>
    }
}
