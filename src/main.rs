use yew::prelude::*;
use yew_router::prelude::*;

mod route;
use route::Route;
mod pages;
use pages::{home::Home};

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::NotFound => {
            html! { <h1 class="mt-5 ml-5 mr-5 title">{ "404 - Page not found" }</h1>}
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <section class="hero is-medium is-primary is-bold">
                <div class="mt-5 ml-5 mr-5 mb-5 px-5">
                    <div class="columns is-vcentered">
                        <div class="column is-2">
                            <h2 class="title is-4">{ "Colin Davis" }</h2>
                        </div>
                    </div>
                </div>
            </section>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
