use crate::components::common::SectionTitle;
use yew::prelude::*;

#[function_component(Skills)]
pub fn skills() -> Html {
    html! {
        <section id="skills">
            <SectionTitle> { "Skills" } </SectionTitle>
        </section>
    }
}
