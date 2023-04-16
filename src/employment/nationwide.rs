use yew::prelude::*;

#[function_component]
pub(crate) fn Nationwide() -> Html {
    html! {
        <>
            <h1 class="text-center">{"Employment at Nationwide"}</h1>
            <div class="my-5 text-center">
                <img src="/img/employment-nationwide.jpg" alt="" style="max-width: 100%" />
            </div>
            <p>{"I'm a proud Nationwider and member of the mainframe team. I get to bring modern technologies and development practices to the mainframe. The legacy nature of the mainframe presents unique challenges when interacting with new technologies, but it's a welcome challenge. My teammates are extremely knowledgeable and are always willing to lend their expertise to help me solve problems."}</p>
            <p>{"Leveraging Python on the mainframe has been awesome. By creating REST APIs using FastAPI, we have been able to quickly and easily develop automation solutions and incorporate them into pipelines and web applications."}</p>
            <p>{"One of the coolest things I have gotten to work on was developing a CI/CD pipeline for COBOL programs that uses the same technologies that distributed applications use. The pipeline allows the developer to use their editor of choice and incorporates GitHub, Jenkins, Artifactory, and Harness to build and deploy applications."}</p>
        </>
    }
}
