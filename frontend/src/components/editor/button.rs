use gloo_console::log;
use reqwest::Client;
use serde_json::json;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub secret: String,
    pub max_reads: i64,
    pub ttl: i64,
    pub on_response: Callback<String>,
}

#[function_component(BackendCallButton)]
pub fn backend_call_button(props: &Props) -> Html {
    let cb = props.on_response.clone();
    let secrets = json!({
        "content": props.secret,
        "max_reads": props.max_reads,
        "ttl": props.ttl
    });

    // Create a callback for the button click
    let onclick = Callback::from(move |_| {
        let cb = cb.clone();
        let secrets = secrets.clone();
        log!(format!("Secrets: {:?}", secrets));

        // Perform the API call when the button is clicked
        spawn_local(async move {
            let client = Client::new();
            match client
                .post("http://localhost:8000/v1/secret")
                .header("Content-Type", "application/json")
                .header("Authorization", "Bearer user1")
                .json(&secrets)
                .send()
                .await
            {
                Ok(res) => match res.text().await {
                    Ok(data) => {
                        log!(format!("Response: {:?}", &data));
                        cb.emit(data);
                    }
                    Err(e) => {
                        log!(format!("Failed to parse response: {:?}", e));
                        eprintln!("Failed to parse response: {:?}", e);
                    }
                },
                Err(e) => {
                    eprintln!("API request failed: {:?}", e);
                }
            }
        });
    });

    html!(
        <button onclick={onclick}
            class="w-full text-md md:text-lg font-medium bg-gray-900 dark:bg-dark-code rounded-md shadow-2xl dark:shadow-gray-400/5 text-gray-50 h-full py-2"
        > {"Share"} </button>
    )
}
