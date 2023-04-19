use leptos::*;
use leptos_router::*;

#[component]
pub(crate) fn Navbar(cx: Scope) -> impl IntoView {
    let pathname = use_location(cx).pathname;

    let (show_education, set_show_education) = create_signal(cx, false);
    let is_show_education = move || show_education();
    let toggle_show_education = move |_| set_show_education.update(|value| *value = !*value);

    let (show_employment, set_show_employment) = create_signal(cx, false);
    let is_show_employment = move || show_employment();
    let toggle_show_employment = move |_| set_show_employment.update(|value| *value = !*value);

    let (show_nav, set_show_nav) = create_signal(cx, false);
    let toggle_show_nav = move |_| match show_nav() {
        true => {
            set_show_nav(false);
            set_show_education(false);
            set_show_employment(false);
        }
        false => {
            set_show_nav(true);
            set_show_education(pathname().starts_with("/education"));
            set_show_employment(pathname().starts_with("/employment"));
        }
    };

    let hide_all_nav = move |_| {
        set_show_nav(false);
        set_show_education(false);
        set_show_employment(false);
    };

    view! { cx,
        <nav class="navbar navbar-dark navbar-expand-lg bg-dark">
            <div class="container-fluid">
                <a class="navbar-brand" href="/" exact=true on:click=hide_all_nav>
                    "wmcgee.tech"
                </a>
                <button
                    class="navbar-toggler"
                    on:click=toggle_show_nav
                    type="button"
                    aria-controls="navbarNavDropdown"
                    aria-expanded=format!("{:?}", show_nav)
                    aria-label="Toggle navigation"
                >
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div
                    class:show=move || show_nav.get()
                    class="collapse navbar-collapse"
                    id="navbarNavDropdown"
                >
                    <ul class="navbar-nav">
                        <li class="nav-item">
                            <a
                                class="nav-link"
                                class:active=move || pathname() == ("/")
                                href="/"
                                on:click=hide_all_nav
                                aria-active=move || if pathname() == ("/") { Some("page") } else { None }
                            >
                                "Home"
                            </a>
                        </li>
                        <li class="nav-item dropdown">
                            <a
                                class="nav-link dropdown-toggle"
                                class:active=move || pathname().starts_with("/education")
                                href="#"
                                on:click=toggle_show_education
                                aria-active=move || if pathname().starts_with("/education") { Some("page") } else { None }
                            >
                                "Education"
                            </a>
                            <ul
                                class="dropdown-menu dropdown-menu-dark"
                                class:show=is_show_education
                            >
                                <li>
                                    <a
                                        class="dropdown-item"
                                        class:active=move || pathname().starts_with("/education/franklin")
                                        href="/education/franklin"
                                        on:click=hide_all_nav
                                        aria-active=move || {
                                            if pathname().starts_with("/education/franklin") { Some("page") } else { None }
                                        }
                                    >
                                        "Franklin"
                                    </a>
                                </li>
                                <li>
                                    <a
                                        class="dropdown-item"
                                        class:active=move || pathname().starts_with("/education/cscc")
                                        href="/education/cscc"
                                        on:click=hide_all_nav
                                        aria-active=move || if pathname().starts_with("/education/cscc") { Some("page") } else { None }
                                    >
                                        "CSCC"
                                    </a>
                                </li>
                                <li>
                                    <a
                                        class="dropdown-item"
                                        class:active=move || pathname().starts_with("/education/dliflc")
                                        href="/education/dliflc"
                                        on:click=hide_all_nav
                                        aria-active=move || if pathname().starts_with("/education/dliflc") { Some("page") } else { None }
                                    >
                                        "DLI-FLC"
                                    </a>
                                </li>
                            </ul>
                        </li>
                        <li class="nav-item dropdown">
                            <a
                                class="nav-link dropdown-toggle"
                                class:active=move || pathname().starts_with("/employment")
                                href="#"
                                on:click=toggle_show_employment
                                aria-active=move || if pathname().starts_with("/employment") { Some("page") } else { None }
                            >
                                "Employment"
                            </a>
                            <ul
                                class="dropdown-menu dropdown-menu-dark"
                                class:show=is_show_employment
                            >
                                <li>
                                    <a
                                        class="dropdown-item"
                                        class:active=move || pathname().starts_with("/employment/nationwide")
                                        href="/employment/nationwide"
                                        on:click=hide_all_nav
                                        aria-active=move || {
                                            if pathname().starts_with("/employment/nationwide") {
                                                Some("page")
                                            } else {
                                                None
                                            }
                                        }
                                    >
                                        "Nationwide"
                                    </a>
                                </li>
                                <li>
                                    <a
                                        class="dropdown-item"
                                        class:active=move || pathname().starts_with("/employment/cscc")
                                        href="/employment/cscc"
                                        on:click=hide_all_nav
                                        aria-active=move || if pathname().starts_with("/employment/cscc") { Some("page") } else { None }
                                    >
                                        "CSCC"
                                    </a>
                                </li>
                                <li>
                                    <a
                                        class="dropdown-item"
                                        class:active=move || pathname().starts_with("/employment/army")
                                        href="/employment/army"
                                        on:click=hide_all_nav
                                        aria-active=move || if pathname().starts_with("/employment/army") { Some("page") } else { None }
                                    >
                                        "U.S. Army"
                                    </a>
                                </li>
                            </ul>
                        </li>
                    </ul>
                </div>
            </div>
        </nav>
    }
}
