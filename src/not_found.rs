use leptos::*;

#[component]
pub(crate) fn NotFound(cx: Scope) -> impl IntoView {
    view! { cx, <h1 class="text-center">"Uh oh, page not found."</h1> }
}
