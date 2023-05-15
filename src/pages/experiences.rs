use crate::{
    components::{
        common::SectionTitle,
        experiences::{Career, CareerData, TimelineComponent},
    },
    define_asset_loader,
};
use yew::prelude::*;

#[function_component(Experiences)]
pub fn experiences() -> Html {
    let careers = get_data();

    html! {
        <section id="experiences">
          <SectionTitle>{ "experiences" } </SectionTitle>
          <TimelineComponent careers={careers} />
        </section>
    }
}

define_asset_loader!(
    "../assets/data/experiences.json",
    Vec<CareerData>,
    Vec<Career>,
    |v: Vec<CareerData>| v.into_iter().map(Into::into).collect()
);
