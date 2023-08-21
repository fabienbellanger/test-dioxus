#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use log::LevelFilter;

fn main() {
    // launch the web app
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let mut counter = use_state(cx, || 0isize);

    cx.render(rsx! {
        div {
            style: "font-size: 1.5em; color: grey",
            "Hello, world!"
        }
        button {
            onclick: move |event| {
                log::info!("Clicked! Event: {event:?}");
                counter += 1;
            },
            "Click me!"
        }
        div {
            style: "font-weight: bold; font-size: 2em",
            "{counter}"
        }
    })
}
 