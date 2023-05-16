use crate::{
    components::{
        common::SectionTitle,
        works::{Detail, WorkData, WorkList},
    },
    define_asset_loader,
};
use yew::prelude::*;

#[function_component(Works)]
pub fn works() -> Html {
    let overviews = get_data();
    html! {
        <section id="works">
          <SectionTitle>{ "works" } </SectionTitle>
          <WorkList overviews={overviews} />
        </section>
    }
}

define_asset_loader!(
    "../assets/data/works.json",
    Vec<WorkData>,
    Vec<Detail>,
    |v: Vec<WorkData>| v.into_iter().map(Into::into).collect()
);
