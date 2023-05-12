use crate::components::{
    self,
    common::SectionTitle,
    skills::{BarGraph, GraphData},
};
use yew::prelude::*;

#[function_component(Skills)]
pub fn skills() -> Html {
    let data = vec![
        GraphData {
            title: "Item 1".into(),
            percentage: 50,
        },
        GraphData {
            title: "Item 2".into(),
            percentage: 75,
        },
        GraphData {
            title: "Item 3".into(),
            percentage: 100,
        },
        // 他のデータ...
    ];

    let data = components::skills::get_data();

    html! {
        <section id="skills">
            <SectionTitle> { "Skills" } </SectionTitle>
            <BarGraph data={data} />
        </section>
    }
}
