use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod nav;
mod pages;
mod route;

use nav::Nav;
use pages::{about::About, experiences::Experiences, skills::Skills, works::Works};
use route::Route;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Nav />
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::About => html! {
            <About />
        },
        Route::Skills => html! {
            <Skills />
        },
        Route::Experiences => html! {
            <Experiences />
        },
        Route::Works => html! {
            <Works />
        },
        Route::NotFound => html! { <h1>{ "not found" } </h1> },
    }
}
