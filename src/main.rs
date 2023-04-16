use yew::prelude::*;
use yew_router::prelude::*;

mod education;
mod employment;
mod footer;
mod navigation;

use footer::Footer;
use navigation::switch;
use navigation::NavBar;
use navigation::Route;

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <header>
                <NavBar />
            </header>
            <main>
                <div class="container mt-3">
                    <Switch<Route> render={switch}/>
                </div>
            </main>
            <footer>
                <Footer />
            </footer>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
