use yew::prelude::*;
use yew_router::prelude::*;

mod education;
mod employment;

#[derive(Clone, PartialEq, Routable)]
pub(crate) enum Route {
    #[at("/")]
    Home,
    #[at("/education/franklin")]
    EducationFranklin,
    #[at("/education/cscc")]
    EducationCscc,
    #[at("/education/dliflc")]
    EducationDliflc,
    #[at("/employment/nationwide")]
    EmploymentNationwide,
    #[at("/employment/cscc")]
    EmploymentCscc,
    #[at("/employment/army")]
    EmploymentArmy,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub(crate) fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { 
            <>
                <h1 class="text-center">{"Home"}</h1>
                <div class="text-center">
                    <img src="img/home.jpg" alt="" style="max-width: 100%;" />
                </div>
            </>
        },
        Route::EducationFranklin => html! { <education::Franklin /> },
        Route::EducationCscc => html! { <education::Cscc /> },
        Route::EducationDliflc => html! { <education::Dliflc /> },
        Route::EmploymentNationwide => html! { <employment::Nationwide /> },
        Route::EmploymentCscc => html! { <employment::Cscc /> },
        Route::EmploymentArmy => html! { <employment::Army /> },
        Route::NotFound => html! { <h1 class="text-center">{"Not Found"}</h1> },
    }
}

#[function_component(NavBar)]
pub(crate) fn nav_bar() -> Html {
    let location = use_location().unwrap();
    let path = location.path();

    let is_show_nav = use_state(|| false);
    let is_show_education = use_state(|| false);
    let is_show_employment = use_state(|| false);
    let hide_nav = {
        let is_show_nav = is_show_nav.clone();
        let is_show_education = is_show_education.clone();
        let is_show_employment = is_show_employment.clone();
        Callback::from(move |_| {
            is_show_nav.set(false);
            is_show_education.set(false);
            is_show_employment.set(false);
        })
    };
    let toggle_nav = {
        let is_show_nav = is_show_nav.clone();
        Callback::from(move |_| is_show_nav.set(!*is_show_nav))
    };
    let toggle_education = {
        let is_show_education = is_show_education.clone();
        Callback::from(move |_| is_show_education.set(!*is_show_education))
    };
    let toggle_employment = {
        let is_show_employment = is_show_employment.clone();
        Callback::from(move |_| is_show_employment.set(!*is_show_employment))
    };

    html! {
        <nav class="navbar navbar-expand-lg bg-dark" data-bs-theme="dark">
            <div class="container-fluid">
                <div onclick={&hide_nav}>
                    <Link<Route> classes={classes!("navbar-brand")} to={Route::Home}>{"wmcgee.tech"}</Link<Route>>
                </div>
                <button class="navbar-toggler" onclick={toggle_nav} type="button" data-target="#navbarNav" aria-controls="navbarNav" aria-expanded={if *is_show_nav { "true" } else { "false" }} aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class={classes!("collapse", "navbar-collapse", if *is_show_nav { "show" } else { "" })} id="navbarNav">
                    <ul class="navbar-nav">
                        <li class="nav-item" onclick={&hide_nav}>
                            <Link<Route> classes={classes!("nav-link", if path == "/" { "active" } else { "" })} to={Route::Home}>{"Home"}</Link<Route>>
                        </li>
                        <li class="nav-item dropdown">
                            <a class={classes!("nav-link", "dropdown-toggle", if path.starts_with("/education") { "active" } else { "" })} id="educationDropdown" role="button" aria-haspopup="true" aria-expanded={if *is_show_education { "true" } else { "false" }} onclick={toggle_education}>
                                {"Education"}
                            </a>
                            <ul class={classes!("dropdown-menu", if *is_show_education { "show" } else { "" })} aria-labelledby="educationDropdown">
                                <li onclick={&hide_nav}>
                                    <Link<Route> classes={classes!("dropdown-item", if path == "/education/franklin" { "active" } else { "" })} to={Route::EducationFranklin}>{"Franklin"}</Link<Route>>
                                </li>
                                <li onclick={&hide_nav}>
                                    <Link<Route> classes={classes!("dropdown-item", if path == "/education/cscc" { "active" } else { "" })} to={Route::EducationCscc}>{"CSCC"}</Link<Route>>
                                </li>
                                <li onclick={&hide_nav}>
                                    <Link<Route> classes={classes!("dropdown-item", if path == "/education/dliflc" { "active" } else { "" })} to={Route::EducationDliflc}>{"DLI-FLC"}</Link<Route>>
                                </li>
                            </ul>
                        </li>
                        <li class="nav-item dropdown">
                            <a class={classes!("nav-link", "dropdown-toggle", if path.starts_with("/employment") { "active" } else { "" })} id="employmentDropdown" role="button" aria-haspopup="true" aria-expanded={if *is_show_education { "true" } else { "false" }} onclick={toggle_employment}>
                                {"Employment"}
                            </a>
                            <ul class={classes!("dropdown-menu", if *is_show_employment { "show" } else { "" })} aria-labelledby="employmentDropdown">
                                <li onclick={&hide_nav}>
                                    <Link<Route> classes={classes!("dropdown-item", if path == "/employment/nationwide" { "active" } else { "" })} to={Route::EmploymentNationwide}>{"Nationwide"}</Link<Route>>
                                </li>
                                <li onclick={&hide_nav}>
                                    <Link<Route> classes={classes!("dropdown-item", if path == "/employment/cscc" { "active" } else { "" })} to={Route::EmploymentCscc}>{"CSCC"}</Link<Route>>
                                </li>
                                <li onclick={&hide_nav}>
                                    <Link<Route> classes={classes!("dropdown-item", if path == "/employment/army" { "active" } else { "" })} to={Route::EmploymentArmy}>{"U.S. Army"}</Link<Route>>
                                </li>
                            </ul>
                        </li>
                    </ul>
                </div>
            </div>
        </nav>
    }
}

#[function_component(Footer)]
fn footer() -> Html {
    html! {
        <div class="mt-3 text-center">
            <p>{"This will be a footer someday..."}</p>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <header>
                <NavBar />
            </header>
            <main>
                <div class="container mt-3">
                    <Switch<Route> render={switch}/>
                </div>
            </main>
            <footer>
                <Footer />
            </footer>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
