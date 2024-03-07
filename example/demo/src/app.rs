use leptos::*;
use leptos_hotkeys::prelude::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::*;

use crate::component::badge::*;
use crate::component::button::*;
use crate::component::card::*;

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
        <script>{include_str!("prism.js")}</script>
        <main _ref=h_ref>
            <ThemeProvider>
                <Router>
                    <Routes>
                        <Route path="/" view=HomePage/>
                        <Route path="/:else" view=ErrorPage/>
                    </Routes>
                </Router>
            </ThemeProvider>
        </main>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let toggle_default = create_rw_signal(false);

    use_hotkeys!(("b", "home") => move |_| {
        if toggle_default.get() {
            toggle_default.set(false);
        } else {
            toggle_default.set(true);
        }
    });

    let buttons = vec![
        (
            "Button sizes",
            view! {
                <div class="flex items-center gap-2">
                    <Button size=ButtonSize::Sm>Small</Button>
                    <Button>Default</Button>
                    <Button size=ButtonSize::Lg>Large</Button>
                    <Button size=ButtonSize::Icon>Icon</Button>
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
        (
            "Conditionally change the button variant",
            view! {
                <Button variant= Signal::derive(move || if toggle_default.get()  { ButtonVariant::Default} else { ButtonVariant::Secondary})>
                    Press B to change variant
                </Button>
            },
        ),
    ];

    view! {
        <Header/>
        <div class="p-8">
            <div class="flex flex-col items-center py-8 gap-4 w-full">
                <h4 class="text-3xl font-semibold p-4">Buttons</h4>

                {buttons
                    .into_iter()
                    .map(|(title, view)| {
                        view! {
                            <Card class="max-w-lg lg:max-w-2xl w-full overflow-none">
                                <CardHeader>
                                    <CardTitle>{title}</CardTitle>
                                </CardHeader>
                                <CardContent>{view}</CardContent>
                                <CardFooter>
                                    <div></div>
                                </CardFooter>
                            </Card>
                        }
                    })
                    .collect::<Vec<_>>()}

            </div>
            <div>
                <div>
                    <h2 class="text-2xl font-bold tracking-tight">Button Code</h2>
                    <p>Leptos code for the Button component</p>
                </div>
                <pre class="whitespace-pre-wrap !rounded-md border">
                    <code class="language-rust !text-sm">
                        {include_str!("component/button.rs")}
                    </code>
                </pre>
            </div>
        </div>
    }
}

#[component]
fn Header() -> impl IntoView {
    let current_theme = use_theme();

    view! {
        <div class="flex items-center justify-between w-full py-4 px-8">
            <div class="flex items-center">
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
                <span> / </span>
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
            <button on:click=move |_| {
                if current_theme.get() == Theme::Dark {
                    current_theme.set(Theme::Light);
                } else {
                    current_theme.set(Theme::Dark);
                }
            }>
                <Show
                    when=Signal::derive(move || current_theme.get() == Theme::Light)
                    fallback=|| {
                        view! {
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                width="24"
                                height="24"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                class="h-4 w-4"
                            >
                                <path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"></path>
                            </svg>
                        }
                    }
                >
                    <svg
                        width="15"
                        height="15"
                        viewBox="0 0 15 15"
                        fill="none"
                        xmlns="http://www.w3.org/2000/svg"
                        class="h-4 w-4"
                    >
                        <path
                            d="M7.5 0C7.77614 0 8 0.223858 8 0.5V2.5C8 2.77614 7.77614 3 7.5 3C7.22386 3 7 2.77614 7 2.5V0.5C7 0.223858 7.22386 0 7.5 0ZM2.1967 2.1967C2.39196 2.00144 2.70854 2.00144 2.90381 2.1967L4.31802 3.61091C4.51328 3.80617 4.51328 4.12276 4.31802 4.31802C4.12276 4.51328 3.80617 4.51328 3.61091 4.31802L2.1967 2.90381C2.00144 2.70854 2.00144 2.39196 2.1967 2.1967ZM0.5 7C0.223858 7 0 7.22386 0 7.5C0 7.77614 0.223858 8 0.5 8H2.5C2.77614 8 3 7.77614 3 7.5C3 7.22386 2.77614 7 2.5 7H0.5ZM2.1967 12.8033C2.00144 12.608 2.00144 12.2915 2.1967 12.0962L3.61091 10.682C3.80617 10.4867 4.12276 10.4867 4.31802 10.682C4.51328 10.8772 4.51328 11.1938 4.31802 11.3891L2.90381 12.8033C2.70854 12.9986 2.39196 12.9986 2.1967 12.8033ZM12.5 7C12.2239 7 12 7.22386 12 7.5C12 7.77614 12.2239 8 12.5 8H14.5C14.7761 8 15 7.77614 15 7.5C15 7.22386 14.7761 7 14.5 7H12.5ZM10.682 4.31802C10.4867 4.12276 10.4867 3.80617 10.682 3.61091L12.0962 2.1967C12.2915 2.00144 12.608 2.00144 12.8033 2.1967C12.9986 2.39196 12.9986 2.70854 12.8033 2.90381L11.3891 4.31802C11.1938 4.51328 10.8772 4.51328 10.682 4.31802ZM8 12.5C8 12.2239 7.77614 12 7.5 12C7.22386 12 7 12.2239 7 12.5V14.5C7 14.7761 7.22386 15 7.5 15C7.77614 15 8 14.7761 8 14.5V12.5ZM10.682 10.682C10.8772 10.4867 11.1938 10.4867 11.3891 10.682L12.8033 12.0962C12.9986 12.2915 12.9986 12.608 12.8033 12.8033C12.608 12.9986 12.2915 12.9986 12.0962 12.8033L10.682 11.3891C10.4867 11.1938 10.4867 10.8772 10.682 10.682ZM5.5 7.5C5.5 6.39543 6.39543 5.5 7.5 5.5C8.60457 5.5 9.5 6.39543 9.5 7.5C9.5 8.60457 8.60457 9.5 7.5 9.5C6.39543 9.5 5.5 8.60457 5.5 7.5ZM7.5 4.5C5.84315 4.5 4.5 5.84315 4.5 7.5C4.5 9.15685 5.84315 10.5 7.5 10.5C9.15685 10.5 10.5 9.15685 10.5 7.5C10.5 5.84315 9.15685 4.5 7.5 4.5Z"
                            fill="currentColor"
                            fill-rule="evenodd"
                            clip-rule="evenodd"
                        ></path>
                    </svg>
                </Show>
            </button>
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
