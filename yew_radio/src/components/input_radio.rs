use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub name: String,
    pub value: String,
    pub onchange: Callback<String>,
}

#[function_component(InputRadio)]
pub fn input_radio(props: &Props) -> Html {
    let callback_onchange = props.onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let value = event.target_unchecked_into::<HtmlInputElement>().value();
        callback_onchange.emit(value);
    });

    html! {
        <div>
            <label>
                {&props.label}
                <input type="radio"
                    name={props.name.clone()}
                    value={props.value.clone()}
                    {onchange} />
            </label>
        </div>
    }
}
