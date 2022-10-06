use super::input_radio::InputRadio;
use yew::prelude::*;

#[function_component(MultipleCustom)]
pub fn multiple_custom() -> Html {
    let onchange = Callback::from(|cat_or_dog: String| {
        gloo::console::log!(cat_or_dog);
    });

    html! {
        <div>
            <h2>{"Multiple Custom Radio Buttons"}</h2>
            <form>
                <InputRadio
                    label="Xilbë"
                    name="cat_or_dog_name"
                    value="xilbe the cat"
                    onchange={onchange.clone()} />
                <InputRadio
                    label="Ranger"
                    name="cat_or_dog_name"
                    value="ranger the dog"
                    onchange={onchange.clone()} />
            </form>
        </div>
    }
}
