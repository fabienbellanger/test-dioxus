#![allow(non_snake_case)]
mod api;
mod counter;

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use api::Api;
use counter::Counter;
use dioxus::prelude::*;
use log::LevelFilter;

fn main() {
    // launch the web app
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger...");
    #[cfg(target_family = "wasm")]
    dioxus_web::launch(App);
    #[cfg(not(target_family = "wasm"))]
    dioxus_desktop::launch_cfg(
        App,
        dioxus_desktop::Config::new().with_custom_head(r#"<link rel="stylesheet" href="tailwind.css">"#.to_string()),
    );
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "container mx-auto p-4",
            h1 {
                class: "text-4xl mb-4",
                "Counter"
            }
            Counter(cx),
            Api(cx),
        }
    })
}
