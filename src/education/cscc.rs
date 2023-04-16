use yew::prelude::*;

#[function_component]
pub(crate) fn Cscc() -> Html {
    html! {
        <>
            <h1 class="text-center">{"Education at CSCC"}</h1>
            <div class="my-5 text-center">
                <img src="/img/education-cscc.jpg" alt="" style="max-width: 100%;" />
            </div>
            <p>{"I began pursuing an Associate's of Applied Science in Cybersecurity at CSCC in January of 2019. After two semesters of studying Cybersecurity, I decided to change my major and pursue the Software Developer Associate's of Applied Science. Fortunately, most of the classes that I completed in my first two semesters counted towards my new major of study. I graduated Magna Cum Laude in December of 2020."}</p>
        </>
    }
}
