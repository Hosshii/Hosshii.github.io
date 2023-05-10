use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(SectionTitle)]
pub fn section_title(props: &Props) -> Html {
    html! {
        <h2>{ for props.children.iter() }</h2>
    }
}
