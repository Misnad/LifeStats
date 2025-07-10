use crate::components::{birth_input, display, name_input};
use crate::models::Man;
use crate::state::AppState;

use chrono::NaiveDate;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(|| AppState::Name);
    let nickname = use_state(String::new);
    let date = use_state(String::new);

    let on_input = {
        let nickname = nickname.clone();
        let date = date.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            match input.name().as_str() {
                "nickname" => nickname.set(input.value()),
                "date" => date.set(input.value()),
                _ => {}
            }
        })
    };

    let on_next = {
        let state = state.clone();
        let nickname = nickname.clone();
        let date = date.clone();

        match (*state).clone() {
            AppState::Name => Callback::from(move |e: SubmitEvent| {
                e.prevent_default();
                if !nickname.is_empty() {
                    state.set(AppState::Birth.clone());
                }
            }),
            AppState::Birth => Callback::from(move |e: SubmitEvent| {
                e.prevent_default();
                let person = Man {
                    nickname: (*nickname).clone(),
                    birth: NaiveDate::parse_from_str((*date).clone().as_str(), "%Y-%m-%d").unwrap(),
                };
                state.set(AppState::Display((person).clone()));
            }),
            _ => Callback::from(move |e: SubmitEvent| {
                e.prevent_default();
                state.set(AppState::Name.clone());
            }),
        }
    };

    let on_key_press = {
        let state = state.clone();
        let nickname = nickname.clone();
        let date = date.clone();

        Callback::from(move |e: KeyboardEvent| match *state {
            AppState::Name => {
                if e.key() == "Enter" && !nickname.is_empty() {
                    state.set(AppState::Birth.clone());
                }
            }
            AppState::Birth => {
                if e.key() == "Enter" && !nickname.is_empty() {
                    // TODO check if date is valid
                    let person = Man {
                        nickname: (*nickname).clone(),
                        birth: NaiveDate::parse_from_str(&(*date).clone(), "%Y-%m-%d").unwrap(),
                    };
                    state.set(AppState::Display((person).clone()));
                }
            }
            _ => {}
        })
    };

    let back_to_input = {
        let state = state.clone();
        let nickname = nickname.clone();
        Callback::from(move |_| {
            nickname.set(String::new());
            state.set(AppState::Name);
        })
    };

    html! {
        <>
            <div class="page-header">
                <h1>{"LifeStats"}</h1>
            </div>
            <div class="main-content"> {
                match (*state).clone() {
                    AppState::Name => html! {
                        <name_input::Name nickname={(*nickname).clone()} oninput={on_input} onkeypress={on_key_press} onsubmit={on_next} />
                    },
                    AppState::Birth => html!{
                        <birth_input::Birth date={(*date).clone()} oninput={on_input} onkeypress={on_key_press.clone()} onsubmit={on_next} />
                    },
                    AppState::Display(man) => html! {
                        <display::Display man={man.clone()} on_back={back_to_input} />
                    }
                }
            }
            </div>
        </>
    }
}
