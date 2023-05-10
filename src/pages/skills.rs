use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Skills)]
pub fn skills() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "Skills" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}
