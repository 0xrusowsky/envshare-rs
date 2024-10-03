use crate::components::editor::{button::BackendCallButton, clipboard::ClipboardComponent};

use super::block::BlockComponent;
use chrono::Utc;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::{prelude::*, Component};

pub enum Msg {
    FocusBlock,
    UpdateMaxReads(i64),
    UpdateTtl(i64),
    UpdateTtlUnit(String),
    UpdateSecret(String),
    UnsealUrl(String),
}

#[derive(Default, Debug)]
pub struct FrameComponent {
    focus_on_render: bool,
    max_reads: i64,
    ttl: i64,
    ttl_unit: String,
    secret: String,
    unseal_url: Option<String>,
}

#[derive(Properties, PartialEq)]
pub struct FrameProps {
    pub focus_ref: NodeRef,
}

impl Component for FrameComponent {
    type Message = Msg;
    type Properties = FrameProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            focus_on_render: true,
            max_reads: 10,
            ttl: 3,
            ttl_unit: "days".to_string(),
            ..Default::default()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FocusBlock => {
                self.focus_on_render = true;
            }
            Msg::UpdateMaxReads(value) => {
                self.max_reads = value;
            }
            Msg::UpdateTtl(value) => {
                self.ttl = value;
            }
            Msg::UpdateTtlUnit(value) => {
                self.ttl_unit = value;
            }
            Msg::UpdateSecret(value) => {
                self.secret = value;
            }
            Msg::UnsealUrl(url) => {
                self.unseal_url = Some(url);
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.input_mode() {
            html! {
                <div style="min-height: 94vh; display: flex; flex-direction: column;" class="text-xs md:text-sm">
                    <div style="min-height: 90px;"/>
                    <div class="subpixel-antialiased bg-gray-900 dark:bg-dark-code rounded-md shadow-2xl dark:shadow-gray-400/5">
                        <BlockComponent textarea_ref={ctx.props().focus_ref.clone()} on_input={ctx.link().callback(|value: String| Msg::UpdateSecret(value))}/>
                    </div>
                    <div class="w-full flex flex-row space-x-2 py-2 text-center h-16">
                        <div class="w-1/2 flex align-center text-center font-mono bg-gray-900 dark:bg-dark-code rounded-md shadow-2xl dark:shadow-gray-400/5 text-gray-500 flex">
                        <p class="w-full text-gray-200 px-2 py-3.5">
                            {"max reads:"}
                        </p>
                        <input
                                type="number"
                                class="w-2/5 no-spinner text-left text-gray-50 bg-transparent appearance-none focus:outline-none focus:ring-0 focus:border-0 active:border-0 py-4"
                                value={self.max_reads.to_string()}
                                onchange={ctx.link().callback(|e: Event| {
                                    let value = e.target_unchecked_into::<HtmlInputElement>().value();
                                    Msg::UpdateMaxReads(value.parse().unwrap_or(10))
                                })}
                            />
                        </div>
                        <div class="w-2/3 flex align-center text-center font-mono bg-gray-900 dark:bg-dark-code rounded-md shadow-2xl dark:shadow-gray-400/5 text-gray-500 flex">
                            <p class="w-full text-gray-200 px-2 py-3.5">{"time to expiry:"}</p>
                            <input type="number"
                                class="w-1/4 no-spinner text-center text-gray-50 bg-transparent appearance-none focus:outline-none focus:ring-0 focus:border-0 active:border-0 py-4"
                                value={self.ttl.to_string()}
                                onchange={ctx.link().callback(|e: Event| {
                                    let value = e.target_unchecked_into::<HtmlInputElement>().value();
                                    Msg::UpdateTtl(value.parse().unwrap_or(7))
                                })}
                            />
                            <select
                                class="w-1/2 text-left text-gray-50 bg-transparent border-0 appearance-none focus:outline-none focus:ring-0 focus:border-0 active:border-0 py-3.5"
                                onchange={ctx.link().callback(|e: Event| {
                                    let value = e.target_unchecked_into::<HtmlInputElement>().value();
                                    Msg::UpdateTtlUnit(value)
                                })}
                            >
                                <option value="minutes" selected={self.ttl_unit == "minutes"}>{"minutes"}</option>
                                <option value="hours" selected={self.ttl_unit == "hours"}>{"hours"}</option>
                                <option value="days" selected={self.ttl_unit == "days"}>{"days"}</option>
                            </select>
                        </div>
                        <BackendCallButton
                            ttl={self.ttl_in_unix()}
                            max_reads={self.max_reads}
                            secret={self.secret.clone()}
                            on_response={ctx.link().callback(Msg::UnsealUrl)}
                        />
                    </div>
                    <p class="px-2 pt-4 text-gray-500">
                        {"Clicking `Share` will generate a new symmetrical key and encrypt your secrets before sending them (only the encrypted data) to the server."}
                    </p>
                </div>
            }
        } else {
            html! {
                <div style="min-height: 95vh; display: flex; flex-direction: column;" class="text-xs md:text-sm items-center justify-center">
                    <h1 class="text-center space-x-2 text-4xl max-md:text-xl font-bold tracking-wide text-gray-800 dark:text-gray-200 py-6">
                        {"Visit this url to unseal the secrets"}
                    </h1>
                    <div class="w-3/4"><div class="flex py-2 rounded-md items-center justify-center bg-gray-200 dark:bg-dark-code space-x-3">
                        <p class="text-center dark:text-gray-50 text-gray-800"> {self.unseal_url.clone().unwrap()} </p>
                        <ClipboardComponent text={self.unseal_url.clone().unwrap()} icon_style={"dark:text-gray-50 text-gray-800"}/>
                    </div></div>
                    <p class="px-2 pt-4 text-gray-500 text-center">
                        {"This url has been created by encoding the uuid, to retrieve the encrypted secret from the database, and the cypher key required to decrypt the secrets."}
                    </p>
                </div>
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render && self.focus_on_render {
            if let Some(textarea) = ctx.props().focus_ref.cast::<HtmlTextAreaElement>() {
                let _ = textarea.focus();
            }
        }
    }
}

impl FrameComponent {
    fn input_mode(&self) -> bool {
        self.unseal_url.is_none()
    }

    fn ttl_in_unix(&self) -> i64 {
        let ttl_in_seconds = match self.ttl_unit.as_str() {
            "minutes" => self.ttl * 60,
            "hours" => self.ttl * 3600,
            "days" => self.ttl * 86400,
            _ => self.ttl,
        };

        Utc::now().timestamp() + ttl_in_seconds
    }
}
