use yew::prelude::*;

#[function_component]
pub(crate) fn Franklin() -> Html {
    html! {
        <>
            <h1 class="text-center">{"Education at Franklin"}</h1>
            <div class="my-5 text-center">
                <img src="/img/education-franklin.jpg" alt="" style="max-width: 100%" />
            </div>
        </>
    }
}
