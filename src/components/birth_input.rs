use yew::prelude::*;
use yew::{Properties, function_component};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub date: String,
    pub oninput: Callback<InputEvent>,
    pub onkeypress: Callback<KeyboardEvent>,
    pub onsubmit: Callback<SubmitEvent>,
}

#[function_component(Birth)]
pub fn birth(props: &Props) -> Html {
    let on_next = {
        let on_next_clone = props.onsubmit.clone();
        Callback::from(move |e: SubmitEvent| on_next_clone.emit(e))
    };

    let on_input = {
        let on_input_clone = props.oninput.clone();
        Callback::from(move |e: InputEvent| on_input_clone.emit(e))
    };

    let on_key_press = {
        let on_key_press_clone = props.onkeypress.clone();
        Callback::from(move |e: KeyboardEvent| on_key_press_clone.emit(e))
    };

    let date = props.date.clone();

    html! {
        <div class="container">
            <div class="content">
                <h1>{"When are you born?"}</h1>
                <form onsubmit={on_next}>
                    <div class="input-group">
                        <input
                            type="date"
                            name="date"
                            value={date}
                            oninput={on_input}
                            onkeydown={on_key_press}
                            class="birth-input"
                        />
                        <button type="submit" class="submit-btn">{"Next"}</button>
                    </div>
                </form>
            </div>
        </div>
    }
}
