mod components;

use components::{multiple_custom::MultipleCustom, multiple_normal::MultipleNormal};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <h1>{"Yew Radio Playground"}</h1>
            <MultipleNormal />
            <MultipleCustom />
        </>
    }
}
