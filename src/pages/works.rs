use crate::{
    components::{
        self,
        common::SectionTitle,
        works::{Detail, WorkData, WorkOverView},
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
          <div class="d-flex gap-3 flex-wrap justify-content-evenly">
            {
              for overviews.iter().map(|v| {
                let detail = v.clone();
                html! {
                  <div class="w-25" style="max-width: 300px; min-width: 200px;">
                    <WorkOverView detail={detail} />
                  </div>
                }
              })
            }
          </div>
        </section>
    }
}

define_asset_loader!(
    "../assets/data/works.json",
    Vec<WorkData>,
    Vec<Detail>,
    |v: Vec<WorkData>| v.into_iter().map(Into::into).collect()
);
