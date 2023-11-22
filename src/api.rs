// #![allow(non_snake_case)]
use dioxus::prelude::*;

const BUTTON_CLASS: &str = "px-6 py-2 text-blue-100 bg-blue-500 hover:bg-blue-700 rounded shadow shadow-blue-500/50";

async fn test_api() -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    client
        .get("https://api.github.com/repos/tokio-rs/axum/releases/latest")
        .header("User-Agent", "Dioxus test")
        .basic_auth("fabienbellanger", Some("eeb45ce7b16798a036ef502b14cc9b8e0931487b"))
        .send()
        .await
        .unwrap()
        .text()
        .await
}

pub fn Api(cx: Scope) -> Element {
    let release_response = use_state(cx, || String::new());
    let get_release = move |_| {
        cx.spawn({
            let release_response = release_response.to_owned();

            async move {
                let response = test_api().await;
                match response {
                    Ok(response) => release_response.set(response),
                    Err(err) => release_response.set(err.to_string()),
                }
            }
        });
    };
    cx.render(rsx! {
        div {
            class: "container mx-auto mt-3",
            h1 {
                class: "text-4xl mb-4",
                "Test Github API"
            }
            button {
                class: BUTTON_CLASS,
                onclick: get_release,
                "Get release info",
            }
            div {
                "{release_response}",
            }
            ApiResult(cx)
        }
    })
}

fn ApiResult(cx: Scope) -> Element {
    let release = use_future(cx, (), |_| test_api());
    cx.render(match release.value() {
        Some(Ok(response)) => rsx! {
            div {
                "Response: {response}"
            }
        },
        Some(Err(_)) => rsx! { div { "Loading release failed" } },
        None => rsx! { div { "Loading..." } },
    })
}
