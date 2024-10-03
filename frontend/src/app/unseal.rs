use crate::components::{editor::static_block::StaticBlockComponent, theme::ThemeComponent};
use chrono::{DateTime, NaiveDate, NaiveDateTime};
use gloo_console::log;
use reqwest::Client;
use serde::Deserialize;
use yew::prelude::*;

pub enum Msg {
    SwitchTheme(bool),
    SetSecret(Secret),
    FetchSecret,
}

#[derive(Properties, Clone, PartialEq)]
pub struct UnsealProps {
    pub encoded_key: String,
}

#[derive(Debug, Clone, PartialEq, Default, Deserialize)]
pub struct Secret {
    uuid: String,
    content: String,
    nonce: String,
    reads_left: i64,
    ttl: i64,
}

pub struct Unseal {
    dark_mode: bool,
    secret: Option<Secret>,
}

impl Component for Unseal {
    type Message = Msg;
    type Properties = UnsealProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::FetchSecret);
        Self {
            dark_mode: true,
            secret: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SwitchTheme(dark_mode) => {
                self.dark_mode = dark_mode;
                true
            }
            Msg::SetSecret(secret) => {
                self.secret = Some(secret);
                true
            }
            Msg::FetchSecret => {
                let encoded_key = ctx.props().encoded_key.clone();
                ctx.link().send_future(async move {
                    let secret = fetch_secret_from_api(&encoded_key).await;
                    Msg::SetSecret(secret)
                });
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={if self.dark_mode { "dark scroll-smooth" } else { "scroll-smooth" }}>
                <div class="w-full flex flex-col bg-gray-100 dark:bg-dark-primary min-h-screen">
                    // Navbar
                    <div class="w-full bg-gray-100 dark:bg-dark-primary" style="position: fixed; top: 0; z-index: 10;">
                        <div class="max-w-md md:max-w-2xl lg:max-w-4xl 2xl:max-w-6xl 4xl:max-w-8xl mx-auto">
                            <div class="flex items-center justify-between px-0 py-4 border-b border-gray-200 dark:border-gray-700">
                                <h1 class="text-2xl max-md:text-lg font-bold tracking-tight text-gray-800 dark:text-gray-200">
                                    {"envshare-rs"}
                                </h1>
                                <div class="flex items-center space-x-2">
                                    <ThemeComponent on_clicked={ctx.link().callback(Msg::SwitchTheme)}/>
                                    // ... (rest of the navbar content)
                                </div>
                            </div>
                        </div>
                    </div>
                    // Main content
                    <div class="px-3 bg-gray-100 dark:bg-dark-primary md:px-0 flex flex-col">
                        <div class="flex flex-col items-center justify-center w-full space-y-8">
                            <div class="w-full max-w-md md:max-w-2xl lg:max-w-4xl 2xl:max-w-6xl 4xl:max-w-8xl 8xl:max-w-10xl">
                                <br />
                                <br />
                                <br />
                                <br />
                                {
                                    if let Some(s) = &self.secret {
                                        html! {
                                            <div class="pb-6">
                                                <StaticBlockComponent content={s.content.clone()}/>
                                                <div class="flex justify-between items-center px-2 pt-2.5 text-gray-500">
                                                    <div class="flex"><p>{"Reads left:"}</p><p class="text-gray-300 pl-2">{s.reads_left}</p></div>
                                                    <div class="flex"><p>{"Expiry time:"}</p><p class="text-gray-300 pl-2">{format!("{}", DateTime::from_timestamp(s.ttl, 0).unwrap())}</p></div>
                                                </div>
                                            </div>
                                        }
                                    } else {
                                        html! {
                                            <p>{"Loading secret..."}</p>
                                        }
                                    }
                                }
                                // Footer
                                // ... (rest of the footer content)
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

async fn fetch_secret_from_api(key: &str) -> Secret {
    let client = Client::new();
    match client
        .get(format!("http://localhost:8000/v1/secret/{}", key))
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer user1")
        .send()
        .await
    {
        Ok(res) => match res.json::<Secret>().await {
            Ok(secret) => secret,
            Err(e) => {
                log!(format!("Failed to parse response: {:?}", e));
                Secret {
                    content: "Error parsing response".to_string(),
                    reads_left: 0,
                    ttl: 0,
                    ..Default::default()
                }
            }
        },
        Err(e) => {
            log!(format!("API request failed: {:?}", e));
            Secret {
                content: "API request failed".to_string(),
                reads_left: 0,
                ttl: 0,
                ..Default::default()
            }
        }
    }
}
