use leptos::*;

#[component]
pub(crate) fn EmploymentCscc(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <h1 class="text-center">"CSCC"</h1>
        <div class="my-4 text-center">
            <img src="../img/employment-cscc.jpg" style="max-width: 100%;" />
        </div>
        <p>{"With the recommendation of two of my instructors, I was hired as a Peer Tutor for Computer Science courses at Columbus State Community College. I tutored introductory Computer Science courses and it was extremely rewarding to help fellow students understand programming."}</p>
        <p>{"I continued tutoring a few Computer Science students after graduating from Columbus State and I am hoping to pursue a bachelor's and then master's degree so I can moonlight as an adjunct instructor for Computer Science courses."}</p>
    }
}
