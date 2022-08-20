use yew::prelude::*;
use yew_router::prelude::*;

mod route;
use route::Route;
mod pages;
use pages::{ projects::Projects, home::Home };

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Projects => html! { <Projects/> },
        Route::Home => html! { <Home/> },
        Route::NotFound => html! { <h1 class="mt-5 ml-5 mr-5 title">{ "404 - Page not found" }</h1>}
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <section class="hero is-medium is-primary is-bold">
                <div class="mt-5 ml-5 mr-5 mb-5 px-5">
                    <div class="columns is-vcentered">
                        <div class="column is-1">
                            <p class="title is-4">{ "Colin Davis" }</p>
                        </div>
                        <div class="column is-1">
                            <Link<Route> to={Route::Home}>
                                { "Home" }
                            </Link<Route>>
                        </div>
                        <div class="column is-1">
                            <Link<Route> to={Route::Projects}>
                                { "Projects" }
                            </Link<Route>>
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