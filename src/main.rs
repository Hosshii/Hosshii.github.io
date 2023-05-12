use yew::prelude::*;

mod components;
mod nav;
mod pages;

use nav::Nav;
use pages::{about::About, experiences::Experiences, skills::Skills, works::Works};

#[function_component(App)]
fn app() -> Html {
    html! {
    <>
      <Nav />
      <div class="container my-3">
        <div class="d-flex flex-column gap-5">
          <About />
          <Skills />
          <Experiences />
          <Works />
        </div>
      </div>
    </>
          }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
