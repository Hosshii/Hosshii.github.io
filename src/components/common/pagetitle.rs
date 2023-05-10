use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(PageTitle)]
pub fn page_title(props: &Props) -> Html {
    html! {
        <h1>{ for props.children.iter() }</h1>
    }
}
