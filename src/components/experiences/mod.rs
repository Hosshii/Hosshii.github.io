use std::rc::Rc;

use crate::components::common::RawHtml;
use serde::Deserialize;
use url::Url;
use yew::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Career {
    pub date: AttrValue,
    pub title: AttrValue,
    pub link: Option<AttrValue>,
    pub description: AttrValue,
}

impl From<CareerData> for Career {
    fn from(value: CareerData) -> Self {
        Self {
            date: AttrValue::from(value.date),
            link: value.link.map(|v| AttrValue::from(v.to_string())),
            title: AttrValue::from(value.title),
            description: AttrValue::from(value.description),
        }
    }
}

#[derive(Debug, Properties, PartialEq)]
struct CareerProps {
    career: Career,
}

#[function_component(CareerComponent)]
fn career_component(props: &CareerProps) -> Html {
    let title = if let Some(ref url) = props.career.link {
        html! {
            <h5><a href={url}>{ &props.career.title }</a></h5>
        }
    } else {
        html! {
            <h5>{ &props.career.title }</h5>
        }
    };
    html! {
        <>
            <div class="timeline__date">{ format!("{} ", &props.career.date) }</div>
            <div class="timeline__details">
                { title }
                <p> <RawHtml html={&props.career.description} /> </p>
            </div>
        </>
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct Careers {
    pub careers: Rc<Vec<Career>>,
}

#[function_component(TimelineComponent)]
pub fn timeline_component(props: &Careers) -> Html {
    html! {
        <div class="timeline">
            <div style={styles::vertical_line(props.careers.len())} />
            { for props.careers.iter().map(|career| html_nested!{<CareerComponent career={career.clone()} />}) }
        </div>
    }
}

mod styles {
    #[allow(unused_imports)]
    use super::*;

    pub fn vertical_line(num_row: usize) -> String {
        format!(
            "grid-column: 2 / span 1;
        grid-row: 1 / span {};
        background-color: #333;
        border: 2px solid #333;
        min-height: 100%;",
            num_row
        )
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct CareerData {
    pub date: String,
    pub title: String,
    pub link: Option<Url>,
    pub description: String,
}
