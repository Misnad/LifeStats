use crate::models::Man;

use chrono::Utc;
use yew::function_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub man: Man,
    pub on_back: Callback<()>,
}

#[function_component(Display)]
pub fn display(props: &Props) -> Html {
    let time_s = props
        .man
        .birth
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc()
        .timestamp();
    let seconds = Utc::now().timestamp() - time_s;
    let hours = seconds / (60 * 60);
    let days = hours / 24;
    let years = days / 365;

    let on_back = {
        let on_back_clone = props.on_back.clone();
        Callback::from(move |_| on_back_clone.emit(()))
    };

    html! {
        <div class="container">
            <div class="content">
                <p>{format!("Hello {}", props.man.nickname)}</p>
                <p>{format!("You lived for:")}</p>
                <p>{format!("{} years", years)}</p>
                <p>{format!("{} days", days)}</p>
                <p>{format!("{} hours", hours)}</p>
                <p>{format!("{} seconds", seconds)}</p>
                <button onclick={on_back} class="back-btn">{"Back"}</button>
            </div>
        </div>
    }
}
