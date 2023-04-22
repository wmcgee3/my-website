use leptos::*;

#[component]
pub(crate) fn EducationCscc(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1 class="text-center">"CSCC"</h1>
        <div class="my-4 text-center">
            <img src="../public/img/education-cscc.jpg" style="max-width: 100%;"/>
        </div>
        <p>
            "I began pursuing an Associate's of Applied Science in Cybersecurity at CSCC in January of 2019.
            After two semesters of studying Cybersecurity, I decided to change my major and pursue the Software Developer Associate's of Applied Science.
            Fortunately, most of the classes that I completed in my first two semesters counted towards my new major of study.
            I graduated Magna Cum Laude in December of 2020."
        </p>
    }
}
