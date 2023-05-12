use crate::components::{
    self,
    common::SectionTitle,
    works::{Detail, WorkOverView},
};
use yew::prelude::*;

#[function_component(Works)]
pub fn works() -> Html {
    let overviews = components::works::get_assets();
    html! {
        <section id="works">
          <SectionTitle>{ "works" } </SectionTitle>
          <div class="d-flex gap-3 flex-wrap justify-content-evenly">
            {
              for overviews.iter().map(|v| {
                let detail = Detail::from(v.clone());
                html! {
                  <div class="w-25" style="max-width: 300px">
                    <WorkOverView detail={detail} />
                  </div>
                }
              })
            }
          </div>
        </section>
    }
}
