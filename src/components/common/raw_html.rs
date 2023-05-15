use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub html: AttrValue,
}

#[function_component(RawHtml)]
pub fn raw_html(props: &Props) -> Html {
    let div = gloo_utils::document().create_element("div").unwrap();
    div.set_inner_html(&props.html.clone());

    Html::VRef(div.into())
}
