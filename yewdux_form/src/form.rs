use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement};
use yew::prelude::*;

#[function_component(BBForm)]
pub fn bb_form() -> Html {
    let onsubmit = Callback::from(|event: FocusEvent| {
        event.prevent_default();
        let form = event.target().unwrap().unchecked_into::<HtmlFormElement>();
        let data = FormData::new_with_form(&form).unwrap();
        let language = data.get("language").as_string().unwrap_or_default();
        let number = data.get("number").as_string().unwrap_or_default();
        let os = data.get("os").as_string().unwrap_or_default();
        let stressed = data.get("stressed").as_string().unwrap_or_default();
        let happy = data.get("happy").as_string().unwrap_or_default();
        gloo::console::log!("event from rust:::", language, number, os, stressed, happy);
    });

    html! {
        <form {onsubmit}>
            <h2>{"The form"}</h2>
            <div>
                <label for="language">{"Favorite Language"}</label>
                <input id="language" name="language" type="text" />
            </div>
            <div>
                <label for="number">{"Favorite Number"}</label>
                <input id="number" name="number" type="number" />
            </div>
            <div>
                <label for="os">{"Favorite OS"}</label>
                <select id="os" name="os">
                    <option value="Mac">{"Mac"}</option>
                    <option value="Windows">{"Windows"}</option>
                    <option value="Linux">{"Linux"}</option>
                </select>
            </div>
            <div>
                <label for="stressed">{"Are you stressed?"}</label>
                <input type="radio" value="stressed" name="stressed" />
                <input type="radio" value="not stressed" name="stressed" />
            </div>
            <div>
                <label for="happy">{"Are you happy?"}</label>
                <input type="checkbox" value="yes I am happy" name="happy" />
            </div>
            <div>
                <button>{"submit"}</button>
            </div>
        </form>
    }
}
