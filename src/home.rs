use leptos::*;

#[component]
pub(crate) fn Home(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <h1 class="text-center">"Home"</h1>
        <div class="my-4 text-center">
            <img src="img/home.jpg" style="max-width: 100%;" />
        </div>
        <p>{"While serving as a Security Manager in the U.S. Army, I learned a ton about physical, personnel, and information security. Wanting to fill the gap in my knowledge, I decided to pursue the Cyber Security A.A.S. at Columbus State Community College. I realized that I had an affinity and passion for programming during my first two semesters of study (and had a few instructors tell me that I was pursuing the wrong major), so I changed my major to the Software Development A.A.S."}</p>
        <p>{"I enjoyed taking more advanced programming courses and worked as a tutor for introductory computer programming courses during my 3rd semester. I worked part-time as an apprentice at Nationwide during my final semester and graduated in December 2020. I was hired to the mainframe team at Nationwide after graduating."}</p>
    }
}
