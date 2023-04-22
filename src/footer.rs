use leptos::*;

#[component]
pub(crate) fn Footer(cx: Scope) -> impl IntoView {
    view! { cx,
        <footer class="bg-dark text-center text-white">
            <div class="container p-4 pb-0">
                <section class="mb-4">
                    <a
                        class="btn btn-outline-light btn-floating m-1"
                        href="https://www.linkedin.com/in/william-mcgee3/"
                        role="button"
                        target="_blank"
                    >
                        <i class="fab fa-linkedin-in"></i>
                    </a>
                    <a
                        class="btn btn-outline-light btn-floating m-1"
                        href="https://github.com/wmcgee3/my-website"
                        role="button"
                        target="_blank"
                    >
                        <i class="fab fa-github"></i>
                    </a>
                </section>
            </div>
            <div class="text-center p-3" style="background-color: rgba(0, 0, 0, 0.2);">
                "© 2023 Copyright: "
                <a class="text-white" href="https://wmcgee.tech/">
                    "wmcgee.tech"
                </a>
            </div>
        </footer>
    }
}
