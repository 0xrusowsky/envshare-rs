use super::types::BlockInput;

use gloo_console::log;
use web_sys::HtmlTextAreaElement;
use yew::{prelude::*, Component};

pub enum Msg {
    InputChanged(BlockInput),
}

#[derive(Properties, PartialEq)]
pub struct BlockProps {
    pub textarea_ref: NodeRef,
}

#[derive(Debug)]
pub struct BlockComponent {
    min_height: i32,
    initialized: bool,
    input: BlockInput,
}

impl Component for BlockComponent {
    type Message = Msg;
    type Properties = BlockProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            min_height: 200,
            initialized: false,
            input: BlockInput::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputChanged(input) => {
                self.initialized = true;
                self.input = input;
                // Manually resize textarea to avoid scrollbars
                if let Some(textarea) = ctx.props().textarea_ref.cast::<HtmlTextAreaElement>() {
                    match textarea.remove_attribute("style") {
                        Ok(_) => {
                            self.min_height = std::cmp::max(200, textarea.client_height() + 10)
                        }
                        Err(_) => log!("Failed to remove style attribute"),
                    }
                    if textarea.scroll_height() > textarea.client_height() {
                        textarea
                            .set_attribute(
                                "style",
                                &format!(
                                    "height: {}px",
                                    std::cmp::max(200, textarea.scroll_height() + 10)
                                ),
                            )
                            .expect("Failed to set style");
                    } else {
                        textarea
                            .set_attribute(
                                "style",
                                &format!(
                                    "height: {}px",
                                    std::cmp::max(200, textarea.client_height() + 10)
                                ),
                            )
                            .expect("Failed to set style");
                    }
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_text_input = ctx.link().callback(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into::<HtmlTextAreaElement>();
            Msg::InputChanged(BlockInput::new(input.value(), input.scroll_height() + 10))
        });

        html! {
            <div class="w-full font-mono text-gray-500 grid h-full grid-cols-[auto,1fr] py-4 px-6">
                <p class="mt-0 text-emerald-400 pb-2 col-span-2 font-bold">{ "secrets" }</p>
                <div class="pr-2 text-left font-mono text-gray-400">
                    { for rows_vec(&self.input.get_value()).iter().map(|num| html! {
                        <div>{if *num < 10 {format!("0{}", num)} else {format!("{}", num)}}</div>
                    }) }
                </div>
                <textarea ref={ctx.props().textarea_ref.clone()}
                    class="w-full h-full font-mono text-gray-50 placeholder-gray-600 bg-transparent border-0 appearance-none resize-none focus:outline-none focus:ring-0 focus:border-0 active:border-0"
                    style={format!("min-height: {}px", self.min_height)}
                    oninput={on_text_input}
                    data-gramm="false"
                    placeholder={"DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres"}>
                </textarea>
            </div>
        }
    }
}

fn rows(text: &str) -> usize {
    text.len() - text.replace("\n", "").len() + 1
}

fn rows_vec(text: &str) -> Vec<usize> {
    (1..=rows(text)).collect()
}
