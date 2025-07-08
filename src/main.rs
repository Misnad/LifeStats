use chrono::{NaiveDate, Utc};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
enum AppState {
    Name,
    Birth,
    Display(Man),
}

#[derive(Debug, Clone, PartialEq)]
struct Man {
    nickname: String,
    birth: NaiveDate,
}

#[function_component(App)]
fn app() -> Html {
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

    let on_submit = {
        let state = state.clone();
        let nickname = nickname.clone();
        let date = date.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if !nickname.is_empty() {
                let person = Man {
                    nickname: (*nickname).clone(),
                    birth: NaiveDate::parse_from_str(&(*date).clone(), "%Y-%m-%d").unwrap(),
                };
                state.set(AppState::Display((person).clone()));
            }
        })
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
                        <div class="container">
                            <div class="content">
                                <h1>{"What is your nickname?"}</h1>
                                <form onsubmit={on_next}>
                                    <div class="input-group">
                                        <input
                                            type="text"
                                            name="nickname"
                                            placeholder="Enter your nickname"
                                            value={(*nickname).clone()}
                                            oninput={on_input}
                                            onkeypress={on_key_press}
                                            class="nickname-input"
                                        />
                                        <button type="submit" class="submit-btn">{"Next"}</button>
                                    </div>
                                </form>
                            </div>
                        </div>
                    },
                    AppState::Birth => html!{
                        <div class="container">
                            <div class="content">
                                <h1>{"What is your asdf?"}</h1>
                                <form onsubmit={on_submit}>
                                    <div class="input-group">
                                        <input
                                            type="date"
                                            name="date"
                                            value={(*date).clone()}
                                            oninput={on_input.clone()}
                                            onkeydown={on_key_press.clone()}
                                            class="birth-input"
                                        />
                                        <button type="submit" class="submit-btn">{"Next"}</button>
                                    </div>
                                </form>
                            </div>
                        </div>
                    },
                    AppState::Display(man) => {
                        let time_s = man.birth.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp();
                        let seconds = Utc::now().timestamp() - time_s;
                        let hours = seconds / (60 * 60);
                        let days = hours / 24;
                        let years = days / 365;
                        html! {
                        <div class="container">
                            <div class="content">
                                <p>{format!("Hello {}", man.nickname)}</p>
                                <p>{format!("You lived for:")}</p>
                                <p>{format!("{} years", years)}</p>
                                <p>{format!("{} days", days)}</p>
                                <p>{format!("{} hours", hours)}</p>
                                <p>{format!("{} seconds", seconds)}</p>
                                <button onclick={back_to_input} class="back-btn">{"Back"}</button>
                            </div>
                        </div>
                        }
                    }
                }
            }
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
