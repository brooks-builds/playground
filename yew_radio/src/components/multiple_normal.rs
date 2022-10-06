use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(MultipleNormal)]
pub fn multiple_normal() -> Html {
    let onchange = Callback::from(|event: Event| {
        let value = event.target_unchecked_into::<HtmlInputElement>().value();
        gloo::console::log!(value);
    });

    html! {
        <form>
            <h2>{"Multiple Normal Radio Buttons"}</h2>
            <div>
                <div>
                    <label for="cat">{"Cat"}</label>
                </div>
                <div>
                    <input type="radio" id="cat" name="cat_or_dog" value="cat" onchange={onchange.clone()} />
                </div>
            </div>
            <div>
                <div>
                    <label for="dog">{"Dog"}</label>
                </div>
                <div>
                    <input type="radio" id="dog" name="cat_or_dog" value="dog" onchange={onchange.clone()} />
                </div>
            </div>
        </form>
    }
}
