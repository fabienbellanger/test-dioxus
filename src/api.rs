// #![allow(non_snake_case)]
use dioxus::prelude::*;

const BUTTON_CLASS: &str = "px-6 py-2 text-blue-100 bg-blue-500 hover:bg-blue-700 rounded shadow shadow-blue-500/50";

async fn test_api(name: &str) -> Result<String, reqwest::Error> {
    dbg!(name);
    let client = reqwest::Client::new();
    client
        .get(format!("https://pokeapi.co/api/v2/pokemon/{name}"))
        .header("User-Agent", "Dioxus test")
        .send()
        .await
        .unwrap()
        .text()
        .await
}

pub fn Api(cx: Scope) -> Element {
    let api_response = use_state(cx, String::new);
    let pokemon_name = use_state(cx, || "pikachu".to_string());
    let get_pokemon = move |_| {
        cx.spawn({
            let api_response = api_response.to_owned();
            let pokemon_name = pokemon_name.clone();

            async move {
                let response = test_api(&pokemon_name).await;
                match response {
                    Ok(response) => api_response.set(response),
                    Err(err) => api_response.set(err.to_string()),
                }
            }
        });
    };
    cx.render(rsx! {
        div {
            class: "container mx-auto mt-3",
            h1 {
                class: "text-4xl mb-4",
                "Test Pokemon API"
            }
            input {
                class: "border",
                value: "{pokemon_name}",
                oninput: move |evt| pokemon_name.set(evt.value.clone()),
            }
            button {
                class: BUTTON_CLASS,
                onclick: get_pokemon,
                "Get Pokemon info",
            }
            div {
                "{api_response}",
            }
            ApiResult(cx),
        }
    })
}

fn ApiResult(cx: Scope) -> Element {
    let pokemon = use_future(cx, (), |_| test_api("pikachu"));
    cx.render(match pokemon.value() {
        Some(Ok(response)) => rsx! {
            div {
                "Response: {response}"
            }
        },
        Some(Err(_)) => rsx! { div { "Loading Pokemon failed" } },
        None => rsx! { div { "Loading..." } },
    })
}
