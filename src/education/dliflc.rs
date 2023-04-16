use yew::prelude::*;

#[function_component]
pub(crate) fn Dliflc() -> Html {
    html! {
        <>
            <h1 class="text-center">{"Education at DLI-FLC"}</h1>
            <div class="my-5 text-center">
                <img src="/img/education-dliflc.jpg" alt="" style="max-width: 100%" />
            </div>
            <p>{"After serving as a medic in the U.S. Army for four years, I volunteered to change my job and become an intelligence analyst. Language school was part of the training for my new job. I was assigned the language of Pashto and and studied at the Defense Language Institute - Foreign language Center at the Presidio of Monterey in Monterey, California. I studied the Pashto language, culture, and history for 16 months. I was awarded a diploma with honors upon completing the program."}</p>
        </>
    }
}
