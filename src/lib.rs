use leptos::*;
use leptos_router::*;

mod education;
mod employment;
mod footer;
mod home;
mod navbar;
mod not_found;

use education::*;
use employment::*;
use footer::*;
use home::*;
use navbar::*;
use not_found::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <header>
                <Navbar/>
            </header>
            <main>
                <div class="container my-4">
                    <Routes>
                        <Route
                            path="/"
                            view=move |cx| {
                                view! { cx, <Home/> }
                            }
                        />
                        <Route
                            path="/education/franklin"
                            view=move |cx| {
                                view! { cx, <EducationFranklin/> }
                            }
                        />
                        <Route
                            path="/education/cscc"
                            view=move |cx| {
                                view! { cx, <EducationCscc/> }
                            }
                        />
                        <Route
                            path="/education/dliflc"
                            view=move |cx| {
                                view! { cx, <EducationDliflc/> }
                            }
                        />
                        <Route
                            path="/employment/nationwide"
                            view=move |cx| {
                                view! { cx, <EmploymentNationwide/> }
                            }
                        />
                        <Route
                            path="/employment/cscc"
                            view=move |cx| {
                                view! { cx, <EmploymentCscc/> }
                            }
                        />
                        <Route
                            path="/employment/army"
                            view=move |cx| {
                                view! { cx, <EmploymentArmy/> }
                            }
                        />
                        <Route
                            path="*"
                            view=move |cx| {
                                view! { cx, <NotFound/> }
                            }
                        />
                    </Routes>
                </div>
            </main>
            <Footer/>
        </Router>
    }
}
