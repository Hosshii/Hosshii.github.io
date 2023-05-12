use std::rc::Rc;

use serde::Deserialize;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct BarGraphProps {
    pub data: Rc<Vec<GraphData>>,
    pub text: Rc<Vec<TextData>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GraphData {
    pub title: AttrValue,
    pub percentage: u32,
}

impl From<GraphDataSer> for GraphData {
    fn from(value: GraphDataSer) -> Self {
        Self {
            title: AttrValue::from(value.title),
            percentage: value.percentage,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextData {
    pub title: AttrValue,
    pub text: AttrValue,
}

impl From<TextDataSer> for TextData {
    fn from(value: TextDataSer) -> Self {
        Self {
            title: AttrValue::from(value.title),
            text: AttrValue::from(value.text),
        }
    }
}

#[function_component(BarGraph)]
pub fn bar_graph(props: &BarGraphProps) -> Html {
    let render_bar = |graph_data: &GraphData| {
        html! {
            <div class="d-flex mb-3 align-items-center">
                <div class="me-3" style="width: 150px;">{ &graph_data.title }</div>
                <div class="flex-grow-1">
                    <div class="progress">
                        <div class="progress-bar" role="progressbar" style={format!("width: {}%;", graph_data.percentage)} aria-valuenow={graph_data.percentage.to_string()} aria-valuemin=0 aria-valuemax=100>
                        </div>
                    </div>
                </div>
            </div>
        }
    };

    let render_text = |text_data: &TextData| {
        html! {
            <div class="d-flex mb-3 align-items-center">
                <div class="me-3" style="width: 150px;">{ text_data.title.clone() }</div>
                <div class="flex-grow-1">
                    { text_data.text.clone() }
                </div>
            </div>
        }
    };

    html! {
        <div>
            { for props.data.iter().map(render_bar) }
            { for props.text.iter().map(render_text) }
        </div>
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GraphDataSer {
    pub title: String,
    pub percentage: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TextDataSer {
    pub title: String,
    pub text: String,
}
