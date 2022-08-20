use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("projects")]
    Projects,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}
