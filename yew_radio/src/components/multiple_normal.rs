use std::ops::Deref;

use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct State {
    pub animal: Animal,
}

#[derive(PartialEq)]
pub enum Animal {
    Dog,
    Cat,
}

#[function_component(MultipleNormal)]
pub fn multiple_normal() -> Html {
    let state = use_state(|| State {
        animal: Animal::Dog,
    });

    let onchange = {
        let state = state.clone();
        Callback::from(move |event: Event| {
            let value = event.target_unchecked_into::<HtmlInputElement>().value();
            let new_state = match value.as_ref() {
                "cat" => State {
                    animal: Animal::Cat,
                },
                "dog" => State {
                    animal: Animal::Dog,
                },
                _ => unreachable!(),
            };
            state.set(new_state);
            gloo::console::log!(value);
        })
    };

    html! {
        <form>
            <h2>{"Multiple Normal Radio Buttons"}</h2>
            <div>
                <div>
                    <label for="cat">{"Cat"}</label>
                </div>
                <div>
                    <input type="radio" id="cat" name="cat_or_dog" value="cat" onchange={onchange.clone()} checked={state.deref().animal == Animal::Cat} />
                </div>
            </div>
            <div>
                <div>
                    <label for="dog">{"Dog"}</label>
                </div>
                <div>
                    <input type="radio" id="dog" name="cat_or_dog" value="dog" onchange={onchange.clone()} checked={state.deref().animal == Animal::Dog} />
                </div>
            </div>
        </form>
    }
}
