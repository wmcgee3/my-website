use leptos::*;

#[component]
pub(crate) fn NotFound(cx: Scope) -> impl IntoView {
    view! { cx, <h1 class="text-center">"404 Not Found"</h1> }
}
