use leptos::*;

#[component]
pub(crate) fn EmploymentArmy(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1 class="text-center">"U.S. Army"</h1>
        <div class="my-4 text-center">
            <img src="../img/employment-army.jpg" style="max-width: 100%;"/>
        </div>
        <p>
            "I enlisted in the United States Army in February of 2009.
            I spent my first enlistment serving as a medic in the 4th Infantry Division at Fort Carson in Colorado Springs, Colorado.
            I deployed to Iraq for 12 months in the spring of 2010.
            After returning from Iraq, I took advantage of an opportunity to undergo retraining and work as an intelligence analyst."
        </p>
        <p>
            "In January of 2013, I moved to Monterey, California and studied the Pashto language for 16 months.
            Upon graduation from language school, I attended further training in Texas and was then stationed in Germany.
            I worked as an intelligence analyst for less than a year before taking over the Battalion's security office.
            I spent the next three years establishing and managing the Battalion's security programs."
        </p>
        <p>
            "In the summer of 2016, my health began to deteriorate.
            After over a year and a half of medical issues and multiple surgeries, I was medically retired in May of 2018."
        </p>
    }
}
