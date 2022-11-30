use web_sys::HtmlInputElement;
use yew::prelude::*;
//use yew::{html, , Html};

#[function_component(App)]
pub fn app() -> Html {
    let counter = use_state(|| 0);
    let step = use_state(|| 1);

    let inc = {
        let counter = counter.clone();
        let step = step.clone();
        Callback::from(move |_| counter.set(*counter + *step))
    };

    let dec = {
        let counter = counter.clone();
        let step = step.clone();
        Callback::from(move |_| counter.set(*counter - *step))
    };

    let change_step = {
        let step = step.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            step.set(input.value().parse::<i32>().unwrap());
        })
    };

    html! {
        <div>
            <p>{*counter}</p>
            <button onclick={inc}>{format!("+{}", (*step).abs())}</button>
            <button onclick={dec}>{format!("-{}", (*step).abs())}</button>
            <div>{"step:"}<input type="number" onchange={change_step} value={(*step).to_string()}/></div>
        </div>
    }
}
