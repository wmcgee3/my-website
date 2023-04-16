use yew::prelude::*;
use yew_router::prelude::*;

use crate::education;
use crate::employment;

mod dropdown;

use dropdown::DropdownLink;
use dropdown::DropdownMenu;

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

#[function_component]
pub(crate) fn NavBar() -> Html {
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
                        <DropdownMenu label="Education" show={*is_show_education} path="/education" onclick={&toggle_education}>
                            <DropdownLink label="Franklin" to={Route::EducationFranklin} onclick={&hide_nav} />
                            <DropdownLink label="CSCC" to={Route::EducationCscc} onclick={&hide_nav} />
                            <DropdownLink label="DLI-FLC" to={Route::EducationDliflc} onclick={&hide_nav} />
                        </DropdownMenu>
                        <DropdownMenu label="Employment" show={*is_show_employment} path="/employment" onclick={&toggle_employment}>
                            <DropdownLink label="Nationwide" to={Route::EmploymentNationwide} onclick={&hide_nav} />
                            <DropdownLink label="CSCC" to={Route::EmploymentCscc} onclick={&hide_nav} />
                            <DropdownLink label="U.S. Army" to={Route::EmploymentArmy} onclick={&hide_nav} />
                        </DropdownMenu>
                    </ul>
                </div>
            </div>
        </nav>
    }
}
