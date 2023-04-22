use leptos::*;

#[component]
pub(crate) fn EducationFranklin(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1 class="text-center">"Franklin University"</h1>
        <div class="my-4 text-center">
            <img src="../public/img/education-franklin.jpg" style="max-width: 100%;"/>
        </div>
        <p></p>
    }
}
