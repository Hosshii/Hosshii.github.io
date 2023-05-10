use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/skills")]
    Skills,
    #[at("/experiences")]
    Experiences,
    #[at("/works")]
    Works,

    #[not_found]
    #[at("/404")]
    NotFound,
}
