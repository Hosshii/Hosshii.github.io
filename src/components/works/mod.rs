use std::rc::Rc;

use serde::Deserialize;
use url::Url;
use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub detail: Detail,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Overview {
    pub title: AttrValue,
    pub img_src: AttrValue,
    pub summary: AttrValue,
    pub tech: Vec<AttrValue>,

    pub js_id: AttrValue,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Detail {
    pub overview: Overview,
    pub detail: AttrValue,
    pub link: Option<Rc<Url>>,
}

impl From<WorkData> for Detail {
    fn from(value: WorkData) -> Self {
        let tech = value
            .tech
            .split(',')
            .map(String::from)
            .map(AttrValue::from)
            .collect();
        Self {
            overview: Overview {
                title: AttrValue::from(value.title),
                img_src: AttrValue::from(value.img_src),
                summary: AttrValue::from(value.summary),
                tech,
                js_id: AttrValue::from(value.js_id),
            },
            detail: AttrValue::from(value.detail),
            link: value.link.map(Rc::new),
        }
    }
}

#[function_component(WorkModal)]
pub fn work_modal(props: &Props) -> Html {
    let overview = &props.detail.overview;
    let label = format!("{}Label", overview.js_id);
    html! {
      <div class="modal fade" id={overview.js_id.clone()} aria-labelledby={label}
      aria-hidden={"true"} >
      <div class="modal-dialog modal-lg modal-dialog-centered">
        <div class="modal-content">
          <div class="modal-header">
            <h4 class="modal-title" id="exampleModalLabel">{overview.title.clone()}</h4>
            <button
              type="button"
              class="btn-close"
              data-bs-dismiss="modal"
              aria-label="Close"
            ></button>
          </div>
          <div class="modal-body">
            <img src={overview.img_src.clone()} class="img-fluid mx-auto w-50 mb-3" style="display: block;"/>
            <div class="overflow-auto text-center">{props.detail.detail.clone()}</div>
          </div>
          <div class="modal-footer">
            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">
              {"Close"}
            </button>
          </div>
        </div>
      </div>
    </div>
          }
}

#[function_component(WorkOverView)]
pub fn work_overview(props: &Props) -> Html {
    let overview = &props.detail.overview;
    let id = format!("#{}", overview.js_id.clone());
    html! {
        <>
          <div class="card h-100 rounded __modal_overview" data-bs-toggle="modal" data-bs-target={id}>
             <img src={overview.img_src.clone()} class="card-img-top" />
             <div class="card-body">
               <h5 class="card-title">{ overview.title.clone() }</h5>
               <p class="card-text">{ overview.summary.clone() }</p>
               <p class="card-text small">{ for overview.tech.iter() }</p>
             </div>
          </div>
          <WorkModal detail={props.detail.clone()}/>
        </>
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct WorkData {
    pub title: String,
    pub link: Option<Url>,
    pub img_src: String,
    pub summary: String,
    pub tech: String,
    pub detail: String,
    pub js_id: String,
}
