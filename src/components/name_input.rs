use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub nickname: String,
    pub onsubmit: Callback<SubmitEvent>,
    pub oninput: Callback<InputEvent>,
    pub onkeypress: Callback<KeyboardEvent>,
}

#[function_component(Name)]
pub fn name(props: &Props) -> Html {
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

    html! {
        <div class="container">
            <div class="content">
                <h1>{"What is your nickname?"}</h1>
                <form onsubmit={on_next}>
                    <div class="input-group">
                        <input
                            type="text"
                            name="nickname"
                            placeholder="Enter your nickname"
                            value={props.nickname.clone()}
                            oninput={on_input}
                            onkeypress={on_key_press}
                            class="nickname-input"
                        />
                        <button type="submit" class="submit-btn">{"Next"}</button>
                    </div>
                </form>
            </div>
        </div>
    }
}
