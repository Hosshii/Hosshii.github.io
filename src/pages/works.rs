use crate::components::common::SectionTitle;
use yew::prelude::*;

#[function_component(Works)]
pub fn works() -> Html {
    html! {
        <section id="works">
          <SectionTitle>{ "works" } </SectionTitle>
        </section>
    }
}
