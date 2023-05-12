use crate::{
    components::{
        common::SectionTitle,
        skills::{BarGraph, GraphData, GraphDataSer, TextData, TextDataSer},
    },
    define_asset_loader,
};
use yew::prelude::*;

#[function_component(Skills)]
pub fn skills() -> Html {
    let data = __data::get_data();
    let text = __text::get_data();

    html! {
        <section id="skills">
            <SectionTitle> { "Skills" } </SectionTitle>
            <BarGraph data={data} text={text} />
        </section>
    }
}

mod __data {
    use super::*;
    define_asset_loader!(
        "../assets/data/skills1.json",
        Vec<GraphDataSer>,
        Vec<GraphData>,
        |v: Vec<GraphDataSer>| v.into_iter().map(Into::into).collect()
    );
}

mod __text {
    use super::*;
    define_asset_loader!(
        "../assets/data/skills2.json",
        Vec<TextDataSer>,
        Vec<TextData>,
        |v: Vec<TextDataSer>| v.into_iter().map(Into::into).collect()
    );
}
