#![allow(non_snake_case)]
use dioxus::prelude::*;

const BUTTON_CLASS: &str = "px-6 py-2 text-blue-100 bg-blue-500 hover:bg-blue-700 rounded shadow shadow-blue-500/50";
const DIV_COUNTER_CLASS: &str = "mt-4 text-2xl font-bold";

pub fn Counter(cx: Scope) -> Element {
    let mut counter = use_state(cx, || 0isize);

    cx.render(rsx! {
        button {
            class: BUTTON_CLASS,
            onclick: move |event| {
                log::info!("Clicked! Event: {event:?}");
                counter += 1;
            },
            "Click me!"
        }
        div {
            class: DIV_COUNTER_CLASS,
            "{counter}"
        }
    })
}
