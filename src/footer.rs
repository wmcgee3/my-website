use leptos::*;

#[component]
pub(crate) fn Footer(cx: Scope) -> impl IntoView {
    view! { cx,
        <footer class="bg-dark text-center text-white">
            <div class="container p-3 pb-0">
                <section class="mb-3">
                    <a
                        class="btn btn-outline-light btn-floating mx-3"
                        href="https://github.com/wmcgee3/my-website"
                        role="button"
                        target="_blank"
                    >
                        <i class="fab fa-github"></i>
                    </a>
                    <a
                        class="btn btn-outline-light btn-floating mx-3"
                        href="https://www.linkedin.com/in/william-mcgee3/"
                        role="button"
                        target="_blank"
                    >
                        <i class="fab fa-linkedin-in"></i>
                    </a>
                </section>
            </div>
            <div class="text-center p-2" style="background-color: rgba(0, 0, 0, 0.2);">
                "© 2023 Copyright: "
                <a class="text-white" href="https://wmcgee.tech/">
                    "wmcgee.tech"
                </a>
            </div>
        </footer>
    }
}
