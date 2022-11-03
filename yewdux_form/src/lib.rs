use yew::{function_component, html};

mod form;

use form::BBForm;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <h1>{"Yewdux form"}</h1>
            <BBForm />
        </main>
    }
}
