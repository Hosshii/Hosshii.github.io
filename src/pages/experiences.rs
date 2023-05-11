use crate::components::common::SectionTitle;
use yew::prelude::*;

#[function_component(Experiences)]
pub fn experiences() -> Html {
    html! {
        <section id="experiences">
          <SectionTitle>{ "experiences" } </SectionTitle>
        </section>
    }
}
